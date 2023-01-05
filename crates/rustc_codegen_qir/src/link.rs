use rustc_codegen_ssa::CodegenResults;
use rustc_middle::bug;
use rustc_session::config::OutputFilenames;
use rustc_session::config::{CrateType, OutputType};
use rustc_session::output::check_file_is_writeable;
use rustc_session::output::invalid_output_for_target;
use rustc_session::output::out_filename;
use rustc_session::utils::NativeLibKind;
use rustc_session::Session;
use std::path::Path;

pub fn link<'a>(
    sess: &'a Session,
    codegen_results: &CodegenResults,
    outputs: &OutputFilenames,
    crate_name: &str,
) {
    let output_metadata = sess.opts.output_types.contains_key(&OutputType::Metadata);
    for &crate_type in sess.crate_types().iter() {
        if (sess.opts.unstable_opts.no_codegen || !sess.opts.output_types.should_codegen())
            && !output_metadata
            && crate_type == CrateType::Executable
        {
            continue;
        }

        if invalid_output_for_target(sess, crate_type) {
            bug!(
                "invalid output type `{:?}` for target os `{}`",
                crate_type,
                sess.opts.target_triple
            );
        }

        for obj in codegen_results
            .modules
            .iter()
            .filter_map(|m| m.object.as_ref())
        {
            check_file_is_writeable(obj, sess);
        }

        if outputs.outputs.should_codegen() {
            let out_filename = out_filename(sess, crate_type, outputs, crate_name);
            match crate_type {
                CrateType::Rlib => {
                    link_rlib(sess, codegen_results, &out_filename);
                }
                //CrateType::Executable | CrateType::Cdylib | CrateType::Dylib => {
                //    link_exe(sess, crate_type, &out_filename, codegen_results);
                //}
                other => {
                    sess.err(&format!("CrateType {:?} not supported yet", other));
                }
            }
        }
    }
}

fn link_rlib(sess: &Session, codegen_results: &CodegenResults, out_filename: &Path) {
    let mut file_list = Vec::<&Path>::new();
    for obj in codegen_results
        .modules
        .iter()
        .filter_map(|m| m.object.as_ref())
    {
        file_list.push(obj);
    }
    for lib in codegen_results.crate_info.used_libraries.iter() {
        match lib.kind {
            NativeLibKind::Static {
                bundle: None | Some(true),
                ..
            } => {}
            NativeLibKind::Static {
                bundle: Some(false),
                ..
            }
            | NativeLibKind::Dylib { .. }
            | NativeLibKind::Framework { .. }
            | NativeLibKind::RawDylib
            | NativeLibKind::LinkArg
            | NativeLibKind::Unspecified => continue,
        }
        if let Some(name) = lib.name {
            sess.err(&format!(
                "Adding native library to rlib not supported yet: {}",
                name
            ));
        }
    }
}
