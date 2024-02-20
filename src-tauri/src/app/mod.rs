pub mod app_context;

#[cfg(target_os="macos")]
mod app_context_mac;

#[cfg(windows)]
mod app_context_win;
