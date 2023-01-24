use crate::QirCodegenBackend;
use rustc_codegen_ssa::{
    back::{
        lto::{LtoModuleCodegen, SerializedModule, ThinModule, ThinShared},
        write::CodegenContext,
    },
    traits::{ModuleBufferMethods, ThinBufferMethods},
};

use crate::qir_errors::qir_fatal_error_wrapper;
use rustc_errors::FatalError;
use rustc_middle::dep_graph::WorkProduct;
use std::ffi::CString;
use std::sync::Arc;

pub struct QirModuleBuffer(pub Vec<u8>);

impl ModuleBufferMethods for QirModuleBuffer {
    fn data(&self) -> &[u8] {
        &self.0
    }
}

pub struct QirThinBuffer(pub Vec<u8>);

impl ThinBufferMethods for QirThinBuffer {
    fn data(&self) -> &[u8] {
        &self.0
    }
}

// In fact this this is simply plumbing, a no-op right now.
pub(crate) fn run_thin(
    _cgcx: &CodegenContext<QirCodegenBackend>,
    modules: Vec<(String, QirThinBuffer)>,
    cached_modules: Vec<(SerializedModule<QirModuleBuffer>, WorkProduct)>,
) -> Result<(Vec<LtoModuleCodegen<QirCodegenBackend>>, Vec<WorkProduct>), FatalError> {
    let mut thin_buffers = Vec::with_capacity(modules.len());
    let mut module_names = Vec::with_capacity(modules.len() + cached_modules.len());

    for (name, buffer) in modules {
        let cname = CString::new(name).map_err(|err| {
            qir_fatal_error_wrapper(&format!("Could not get module name: {} ", err.to_string()))
        })?;

        thin_buffers.push(buffer);
        module_names.push(cname);
    }

    let mut serialized_modules = Vec::with_capacity(cached_modules.len());

    for (sm, wp) in cached_modules {
        let _slice_u8 = sm.data();
        serialized_modules.push(sm);
        module_names.push(CString::new(wp.cgu_name).map_err(|err| {
            qir_fatal_error_wrapper(&format!(
                "Could not get codegen unit name: {} ",
                err.to_string()
            ))
        })?);
    }

    let shared = Arc::new(ThinShared {
        data: (),
        thin_buffers,
        serialized_modules,
        module_names,
    });

    let opt_jobs = shared
        .module_names
        .iter()
        .enumerate()
        .map(|(module_idx, _)| {
            LtoModuleCodegen::Thin(ThinModule {
                shared: shared.clone(),
                idx: module_idx,
            })
        })
        .collect();

    Ok((opt_jobs, vec![]))
}
