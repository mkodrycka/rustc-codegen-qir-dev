#![feature(rustc_private)]
#![feature(extern_types)]

// The below are private rustc crates availble behind the `rustc_private` feature.
extern crate rustc_abi;
extern crate rustc_ast;
extern crate rustc_codegen_ssa;
extern crate rustc_const_eval;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

// This prevents duplicating functions and statics
// that are already part of the host rustc process.
extern crate rustc_driver;

use inkwell::context::Context;
use rustc_ast::expand::allocator::AllocatorKind;
use rustc_codegen_ssa::{
    back::{
        lto::{LtoModuleCodegen, SerializedModule, ThinModule},
        write::{CodegenContext, FatLTOInput, ModuleConfig, OngoingCodegen},
    },
    traits::{CodegenBackend, ExtraBackendMethods, WriteBackendMethods},
    CodegenResults, CompiledModule, ModuleCodegen, ModuleKind,
};
use rustc_errors::{ErrorGuaranteed, FatalError, Handler};
use rustc_hash::FxHashMap;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::{query, TyCtxt},
};
use rustc_session::{
    config::{Options, OutputFilenames, OutputType},
    Session,
};
use rustc_target::spec::{Target, TargetOptions, TargetTriple};

use log::{debug, info, warn};
use serde::{
    de::{value::Error as SerdeError, Deserialize as DeserializeTrait, IntoDeserializer},
    Deserialize, Serialize,
};
use std::any::Any;
use std::io::Write;
use std::{fs::File, sync::Arc};

mod builder;
mod codegen;
mod lto;

use crate::codegen::QirCodegenCompiler;
use crate::lto::{QirModuleBuffer, QirThinBuffer};

use rustc_session::{config::CrateType, output::out_filename};

mod qir_errors;
use crate::qir_errors::qir_fatal_error_wrapper;

const QIR_ARCH: &'static str = "qir";

// codegen dylib entrypoint
#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    let _ = std::panic::take_hook();

    info!("::QirCodegenBackend is starting...");

    Box::new(QirCodegenBackend)
}

/// Code generation backend for QIR instructions.
///
/// QIR can be expressed differently based on the supplied [QirProfile].

#[derive(Debug, Clone)]
pub struct QirCodegenBackend;

impl CodegenBackend for QirCodegenBackend {
    fn init(&self, sess: &Session) {
        // Initialize the logging library
        env_logger::init();

        info!("::CodegenBackend Initializing the QIR codegen backend");
    }

    fn codegen_crate(
        &self,
        tcx: TyCtxt<'_>,
        metadata: EncodedMetadata,
        need_metadata_module: bool,
    ) -> Box<dyn Any> {
        Box::new(rustc_codegen_ssa::base::codegen_crate(
            Self,
            tcx,
            tcx.sess
                .opts
                .cg
                .target_cpu
                .clone()
                .unwrap_or_else(|| tcx.sess.target.cpu.to_string()),
            metadata,
            need_metadata_module,
        ))
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn Any>,
        sess: &Session,
        _outputs: &OutputFilenames,
    ) -> Result<(CodegenResults, FxHashMap<WorkProductId, WorkProduct>), ErrorGuaranteed> {
        let (codegen_results, work_products) = ongoing_codegen
            .downcast::<OngoingCodegen<Self>>()
            .expect("Expected OngoingCodegen, found Box<Any>")
            .join(sess);

        sess.compile_status()?;

        Ok((codegen_results, work_products))
    }

    fn link(
        &self,
        sess: &Session,
        codegen_results: CodegenResults,
        outputs: &OutputFilenames,
    ) -> Result<(), ErrorGuaranteed> {
        debug!("::CodegenBackend Linking");

        let crate_name = codegen_results.crate_info.local_crate_name;
        for &crate_type in sess.opts.crate_types.iter() {
            if crate_type != CrateType::Rlib {
                sess.fatal(&format!("Crate type is {:?}", crate_type));
            }
            let output_name = out_filename(sess, crate_type, &outputs, crate_name);
            let mut out_file = ::std::fs::File::create(output_name).unwrap();
            write!(out_file, "This has been \"compiled\" successfully.").unwrap();
        }
        Ok(())
    }

    fn provide(&self, providers: &mut query::Providers) {
        // TODO: We can probably parse the QIR profile here instead of in `target_override`...
        providers.global_backend_features = |_tcx, ()| vec![];
    }

    // Note: This is called _before_ init, thus we can't log :(
    fn target_override(&self, opts: &Options) -> Option<Target> {
        // Here we extract the target triple supplied and make sure that it is a valid option. We return None
        //  otherwise.
        let triple_parts = match &opts.target_triple {
            TargetTriple::TargetTriple(triple) => triple,
            TargetTriple::TargetJson { triple, .. } => triple,
        }
        .split("-")
        .collect::<Vec<&str>>();

        // Ensure that we have a valid triple
        if triple_parts.len() != 3 {
            return None;
        }

        // Match to a valid QIR profile. Invalid profiles will short out.
        let raw_profile = triple_parts[2];
        let profile: Result<QirProfile, SerdeError> =
            QirProfile::deserialize(raw_profile.into_deserializer());
        match profile {
            Ok(p) => p,
            Err(_) => return None,
        };

        Some(Target {
            arch: QIR_ARCH.into(),

            // Refer to https://llvm.org/docs/LangRef.html#data-layout
            // TODO: What should these be? We aren't packing the structure at all, so maybe we can clone
            //  the host system's data layout / pointer width?
            data_layout: "e".into(),
            pointer_width: 64,

            // We will model the target triple using the convention of ARCH-VENDOR-PROFILE. The
            //  arch will always be constant, but the profile should be one which QIR supports.
            llvm_target: format!("{}-unknown-{}", QIR_ARCH, raw_profile).into(),

            options: generate_qir_target_options(),
        })
    }
}

impl WriteBackendMethods for QirCodegenBackend {
    type Module = Vec<u8>;
    type TargetMachine = ();
    type ModuleBuffer = QirModuleBuffer;
    type ThinData = ();
    type ThinBuffer = QirThinBuffer;

    fn run_link(
        _cgcx: &CodegenContext<Self>,
        _diag_handler: &Handler,
        _modules: Vec<ModuleCodegen<Self::Module>>,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError> {
        todo!()
    }

    fn run_fat_lto(
        _: &CodegenContext<Self>,
        _: Vec<FatLTOInput<Self>>,
        _: Vec<(SerializedModule<Self::ModuleBuffer>, WorkProduct)>,
    ) -> Result<LtoModuleCodegen<Self>, FatalError> {
        todo!()
    }

    fn run_thin_lto(
        cgcx: &CodegenContext<Self>,
        modules: Vec<(String, Self::ThinBuffer)>,
        cached_modules: Vec<(SerializedModule<Self::ModuleBuffer>, WorkProduct)>,
    ) -> Result<(Vec<LtoModuleCodegen<Self>>, Vec<WorkProduct>), FatalError> {
        lto::run_thin(cgcx, modules, cached_modules)
    }

    fn print_pass_timings(&self) {
        todo!()
    }

    unsafe fn optimize(
        _: &CodegenContext<Self>,
        _: &Handler,
        _: &ModuleCodegen<Self::Module>,
        _: &ModuleConfig,
    ) -> Result<(), FatalError> {
        warn!("::WriteBackendMethods Optimizations are no-ops");
        Ok(())
    }

    unsafe fn optimize_thin(
        _cgcx: &CodegenContext<Self>,
        thin_module: ThinModule<Self>,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError> {
        debug!("::WriteBackendMethods Optimize Thin");
        todo!()
    }

    fn optimize_fat(
        _: &CodegenContext<Self>,
        _: &mut ModuleCodegen<Self::Module>,
    ) -> Result<(), FatalError> {
        todo!()
    }

    unsafe fn codegen(
        cgcx: &CodegenContext<Self>,
        _diag_handler: &Handler,
        module: ModuleCodegen<Self::Module>,
        _config: &ModuleConfig,
    ) -> Result<CompiledModule, FatalError> {
        debug!("::WriteBackendMethods Codegen");
        let path = cgcx
            .output_filenames
            .temp_path(OutputType::Object, Some(&module.name));

        File::create(&path)
            .map_err(|err| {
                qir_fatal_error_wrapper(&format!(
                    "Could not get {}: {}",
                    &path.display(),
                    err.to_string()
                ))
            })?
            .write_all(&module.module_llvm)
            .map_err(|err| {
                qir_fatal_error_wrapper(&format!("Could not write: {}", err.to_string()))
            })?;

        Ok(CompiledModule {
            name: module.name,
            kind: module.kind,
            object: Some(path),
            dwarf_object: None,
            bytecode: None,
        })
        // todo!()
    }

    fn prepare_thin(module: ModuleCodegen<Self::Module>) -> (String, Self::ThinBuffer) {
        (module.name, QirThinBuffer(module.module_llvm))
    }

    fn serialize_module(module: ModuleCodegen<Self::Module>) -> (String, Self::ModuleBuffer) {
        (module.name, QirModuleBuffer(module.module_llvm))
    }
}

impl ExtraBackendMethods for QirCodegenBackend {
    fn codegen_allocator<'tcx>(
        &self,
        tcx: TyCtxt<'tcx>,
        module_name: &str,
        kind: AllocatorKind,
        alloc_handler_error_kind: AllocatorKind,
    ) -> Self::Module {
        debug!("::ExtraBackendMethods Codegen Allocator");
        todo!()
    }

    fn compile_codegen_unit(
        &self,
        tcx: TyCtxt<'_>,
        cgu_name: rustc_span::Symbol,
    ) -> (ModuleCodegen<Self::Module>, u64) {
        debug!("::ExtraBackendMethods Compile Codegen Unit");

        let cgu = tcx.codegen_unit(cgu_name);
        let context = Context::create();
        let codegen = QirCodegenCompiler::new(tcx, cgu, &context);

        let module = match codegen.compile() {
            Ok(m) => m,
            Err(e) => tcx
                .sess
                .fatal(format!("could not compile module {cgu_name}: {e}")),
        };

        // Inform the compiler of the ongoing work and its cost
        // TODO: How can we measure the cost?
        (
            ModuleCodegen {
                name: cgu_name.to_string(),
                module_llvm: module.write_bitcode_to_memory().as_slice().to_vec(),
                kind: ModuleKind::Regular,
            },
            0,
        )
    }

    fn target_machine_factory(
        &self,
        sess: &Session,
        opt_level: rustc_session::config::OptLevel,
        target_features: &[String],
    ) -> rustc_codegen_ssa::back::write::TargetMachineFactoryFn<Self> {
        debug!("::ExtraBackendMethods Target Machine Factory");

        // TODO: What should this do? It's apparently passed the result of the `CodegenBackend::provide` method...
        Arc::new(|_| Ok(()))
    }
}

/// Generate target options for QIR.
///
/// These options correspond to valid compiler actions supported by the QIR spec.
fn generate_qir_target_options() -> TargetOptions {
    let mut options = TargetOptions::default();

    // Disable atomics
    options.max_atomic_width = Some(0);

    // Allow for dylibs
    options.dynamic_linking = true;

    options
}

/// A valid QIR profile.
///
/// Refer to https://github.com/qir-alliance/qir-spec/tree/spec_update/specification/under_development/profiles
/// for more info.
// TODO: Make a common library with data structures to allow for sharing across the various
//  components.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum QirProfile {
    #[serde(rename = "base")]
    Base,
}
