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

pub struct QirModuleBuffer(pub Vec<u32>);

impl ModuleBufferMethods for QirModuleBuffer {
    fn data(&self) -> &[u8] {
        from_binary_to_byte_array(&self.0)
    }
}

pub struct QirThinBuffer(pub Vec<u32>);

impl ThinBufferMethods for QirThinBuffer {
    fn data(&self) -> &[u8] {
        from_binary_to_byte_array(&self.0)
    }
}

/// Convert a binary being stored as 32 bit words into a byte array.
#[inline]
pub fn from_binary_to_byte_array(bin: &[u32]) -> &[u8] {
    // This is an unsafe operation since we don't have assurance that
    // 1) given pointer is valid for len elements,
    // 2) the lifetime inferred is an appropriate lifetime for the returned slice.

    // See more details about std::slice::from_raw_parts's safety:
    // https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html
    unsafe {
        std::slice::from_raw_parts(bin.as_ptr().cast(), bin.len() * std::mem::size_of::<u32>())
    }
}

/// Convert a regular byte array into a binary of 32 bit words.
#[inline]
pub fn from_byte_array_to_binary(bytes: &[u8]) -> Result<&[u32], FatalError> {
    // This should fail if the input is not `% sizeof(u32)`.
    if bytes.len() % std::mem::size_of::<u32>() != 0 {
        return Err(FatalError);
    }

    // This is an unsafe operation since we don't have assurance that
    // 1) given pointer is valid for len elements,
    // 2) the lifetime inferred is an appropriate lifetime for the returned slice.

    // See more details about std::slice::from_raw_parts's safety:
    // https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html
    #[allow(clippy::size_of_in_element_count)]
    Ok(unsafe {
        std::slice::from_raw_parts(
            bytes.as_ptr().cast(),
            bytes.len() / std::mem::size_of::<u32>(),
        )
    })
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
