use log::error;
use rustc_errors::FatalError;

pub fn qir_fatal_error_wrapper(msg: &str) -> FatalError {
    error!("{}", msg);
    FatalError {}
}
