use once_cell::sync::Lazy;
use std::sync::Mutex;
pub(super) static CLIPBOARD_CONTEXT_LOCK: Lazy<Mutex<ClipaAppContext>> =
    Lazy::new(|| Mutex::new(ClipaAppContext::new()));
pub struct ClipaAppContext {
    pub(crate) invoking_app_id: String,
}