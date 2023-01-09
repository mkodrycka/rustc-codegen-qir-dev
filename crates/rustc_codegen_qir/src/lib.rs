#![feature(rustc_private)]
#![feature(extern_types)]

// The below are private rustc crates availble behind the `rustc_private` feature.
extern crate rustc_ast;
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

// This prevents duplicating functions and statics
// that are already part of the host rustc process.
extern crate rustc_driver;

use rustc_codegen_ssa::{
    back::{
        lto::{LtoModuleCodegen, SerializedModule, ThinModule},
        write::{CodegenContext, FatLTOInput, ModuleConfig, OngoingCodegen},
    },
    traits::{CodegenBackend, ExtraBackendMethods, WriteBackendMethods},
    CodegenResults, CompiledModule, CrateInfo, ModuleCodegen, ModuleKind,
};
use rustc_errors::{ErrorGuaranteed, FatalError, Handler};
use rustc_hash::FxHashMap;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::TyCtxt,
};
use rustc_session::{
    config::{Options, OutputFilenames, OutputType},
    cstore::MetadataLoaderDyn,
    Session,
};
use rustc_target::spec::{Target, TargetOptions, TargetTriple};

use log::{debug, error, info, warn};
use serde::{
    de::{value::Error as SerdeError, Deserialize as DeserializeTrait, IntoDeserializer},
    Deserialize, Serialize,
};
use std::ffi::CString;
use std::fs::{create_dir_all, File};
use std::io::Write;

mod lto;
use crate::lto::{
    from_binary_to_byte_array, from_byte_array_to_binary, QirModuleBuffer, QirThinBuffer,
};

use rustc_session::{config::CrateType, output::out_filename};

mod qir_errors;
use crate::qir_errors::qir_fatal_error_wrapper;

const QIR_ARCH: &'static str = "qir";

// codegen dylib entrypoint
#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    let _ = std::panic::take_hook();

    info!("::QirCodegenBackend is starting...");

    Box::new(QirCodegenBackend::default())
}

/// Code generation backend for QIR instructions.
///
/// QIR can be expressed differently based on the supplied [QirProfile].

#[derive(Default, Clone)]
pub struct QirCodegenBackend {}

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
    ) -> Box<dyn std::any::Any> {
        debug!("::CodegenBackend Codegen crate");

        Box::new(CodegenResults {
            modules: vec![],
            allocator_module: None,
            metadata_module: None,
            metadata,
            crate_info: CrateInfo::new(tcx, QIR_ARCH.into()),
        })
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn std::any::Any>,
        sess: &Session,
        outputs: &OutputFilenames,
    ) -> Result<(CodegenResults, FxHashMap<WorkProductId, WorkProduct>), ErrorGuaranteed> {
        debug!("::CodegenBackend Join Codegen");

        let codegen_results = ongoing_codegen
            .downcast::<CodegenResults>()
            .expect("in join_codegen: ongoing_codegen is not a CodegenResults");
        Ok((*codegen_results, FxHashMap::default()))
    }

    fn link(
        &self,
        sess: &Session,
        codegen_results: CodegenResults,
        outputs: &OutputFilenames,
    ) -> Result<(), ErrorGuaranteed> {
        debug!("::CodegenBackend Linking");

        let crate_name = codegen_results.crate_info.local_crate_name.as_str();
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
    type Module = Vec<u32>;
    type TargetMachine = ();
    type ModuleBuffer = QirModuleBuffer;
    type Context = ();
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
        todo!()
    }

    unsafe fn optimize_thin(
        _cgcx: &CodegenContext<Self>,
        thin_module: ThinModule<Self>,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError> {
        let module = ModuleCodegen {
            module_llvm: from_byte_array_to_binary(thin_module.data())
                .map_err(|err| {
                    qir_fatal_error_wrapper(&format!(
                        "Got the wrong input size: {} ",
                        err.to_string()
                    ))
                })?
                .to_vec(),
            name: thin_module.name().to_string(),
            kind: ModuleKind::Regular,
        };
        Ok(module)
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
        let path = cgcx
            .output_filenames
            .temp_path(OutputType::Object, Some(&module.name));

        let qir_module = from_binary_to_byte_array(&module.module_llvm);
        File::create(&path)
            .map_err(|err| {
                qir_fatal_error_wrapper(&format!(
                    "Could not get {}: {}",
                    &path.display(),
                    err.to_string()
                ))
            })?
            .write_all(qir_module)
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
    }

    fn prepare_thin(module: ModuleCodegen<Self::Module>) -> (String, Self::ThinBuffer) {
        (module.name, QirThinBuffer(module.module_llvm))
    }

    fn serialize_module(module: ModuleCodegen<Self::Module>) -> (String, Self::ModuleBuffer) {
        (module.name, QirModuleBuffer(module.module_llvm))
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
#[derive(Serialize, Deserialize)]
enum QirProfile {
    #[serde(rename = "base")]
    Base,
}
