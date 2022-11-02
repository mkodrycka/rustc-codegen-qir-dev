#![feature(rustc_private)]

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

use rustc_codegen_ssa::{
    back::{
        lto::{LtoModuleCodegen, SerializedModule, ThinModule},
        write::{CodegenContext, FatLTOInput, ModuleConfig, OngoingCodegen},
    },
    traits::{CodegenBackend, ExtraBackendMethods, WriteBackendMethods},
    CodegenResults, CompiledModule, CrateInfo, ModuleCodegen,
};
use rustc_errors::{ErrorGuaranteed, FatalError, Handler};
use rustc_hash::FxHashMap;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::TyCtxt,
};
use rustc_session::{
    config::{OutputFilenames, Options},
    cstore::MetadataLoaderDyn,
    Session,
};
use rustc_target::spec::{Target, TargetOptions, TargetTriple};

use log::{
    debug,
    error,
    info,
    warn,
};
use serde::{
    de::{
        value::Error as SerdeError,

        Deserialize as DeserializeTrait,
        IntoDeserializer
    },

    Deserialize,
    Serialize,
};

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
#[derive(Default)]
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
            crate_info: CrateInfo::new(tcx, "qir".to_string()),
        })
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn std::any::Any>,
        sess: &Session,
        outputs: &OutputFilenames,
    ) -> Result<(CodegenResults, FxHashMap<WorkProductId, WorkProduct>), ErrorGuaranteed> {
        debug!("::CodegenBackend Joining codegen");

        todo!()
    }

    fn link(
        &self,
        sess: &Session,
        codegen_results: CodegenResults,
        outputs: &OutputFilenames
    ) -> Result<(), ErrorGuaranteed> {
        debug!("::CodegenBackend Linking");

        todo!()
    }

    // Note: This is called _before_ init, thus we can't log :(
    fn target_override(&self, opts: &Options) -> Option<Target> {
        // Here we extract the target triple supplied and make sure that it is a valid option. We return None
        //  otherwise.
        let triple_parts = match &opts.target_triple {
            TargetTriple::TargetTriple(triple)      => triple,
            TargetTriple::TargetJson { triple, .. } => triple,
        }.split("-").collect::<Vec<&str>>();

        // Ensure that we have a valid triple
        if triple_parts.len() != 3 {
            return None;
        }

        // Match to a valid QIR profile. Invalid profiles will short out.
        let raw_profile = triple_parts[2];
        let profile: Result<QirProfile, SerdeError> = QirProfile::deserialize(raw_profile.into_deserializer());
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

            // TODO: Fill this in with options allowing for dynamic libraries. The main issue is that
            //  the [TargetOptions] struct contains private members, so it cannot be constructed manually.
            options: TargetOptions::default(),
        })
    }
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
