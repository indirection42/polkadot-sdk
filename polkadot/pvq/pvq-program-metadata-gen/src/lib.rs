//! PVQ program metadata generator primitives.
//!
//! This crate is the library backend used by the `pvq-program-metadata-gen` CLI.
//! It parses a PVQ program's Rust source (currently `src/main.rs`), finds the
//! `#[program]` module, and collects function signatures annotated with:
//!
//! - `#[program::entrypoint]`
//! - `#[program::extension_fn(extension_id = ..., fn_index = ...)]`
//!
//! The CLI then builds a temporary crate that serializes the collected metadata
//! and writes it to the output directory.
//!
//! Note: the binary expects to run in an environment where Cargo sets
//! `CARGO_PKG_NAME` (e.g. from a `build.rs`). If you run it manually, you'll need
//! to set that variable yourself.

/// Extension identifier used by PVQ extension functions.
pub type ExtensionId = u64;

/// Function index used to disambiguate functions within an extension.
pub type FnIndex = u8;
mod features;
mod helper;
pub use features::{extract_features, get_active_features};
mod manifest;
pub use manifest::create_manifest;
mod metadata_gen;
pub use metadata_gen::metadata_gen_src;
