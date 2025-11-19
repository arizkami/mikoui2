// MikoCore - Core functionality for Rabital
// This crate will contain shared core functionality

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
