//! Procedural macros for the PVQ extension system.
//!
//! Most users should depend on [`pvq_extension`], which re-exports these macros.
//!
//! ## What the macros generate
//!
//! - `#[extension_decl]` (on a `mod`) generates an enum `Functions<Impl>` representing calls into a
//!   single extension trait, plus helper functions like `extension_id()` and `metadata()`.
//! - `#[extensions_impl]` (on a `mod`) collects multiple `#[extensions_impl::extension] impl`s into:
//!   - a `pub type Extensions = (...)` tuple of the generated `Functions<Impl>` call types, and
//!   - a `pub fn metadata() -> pvq_extension::metadata::Metadata` for all extensions in the module.
//!
//! ## Sketch
//!
//! ```no_run
//! use pvq_extension::{extension_decl, extensions_impl};
//!
//! #[extension_decl]
//! pub mod ext_math {
//!     #[extension_decl::extension]
//!     pub trait Math {
//!         fn add(a: u32, b: u32) -> u32;
//!     }
//! }
//!
//! #[extensions_impl]
//! pub mod extensions {
//!     use super::ext_math;
//!
//!     #[extensions_impl::impl_struct]
//!     pub struct ExtensionImpl;
//!
//!     #[extensions_impl::extension]
//!     impl ext_math::Math for ExtensionImpl {
//!         fn add(a: u32, b: u32) -> u32 { a + b }
//!     }
//! }
//!
//! let _id = ext_math::extension_id();
//! let _one = ext_math::metadata::<extensions::ExtensionImpl>();
//! let _all = extensions::metadata();
//! ```
use proc_macro::TokenStream;
mod extension_decl;
mod extensions_impl;
pub(crate) mod utils;

/// Declare a PVQ extension interface.
///
/// Attach `#[extension_decl]` to a `mod` that contains one or more traits annotated with
/// `#[extension_decl::extension]`.
///
/// For each extension trait, the macro generates (inside that module):
/// - `pub enum Functions<Impl: Trait>`: a SCALE-`Encode`/`Decode` call enum, and
/// - implementations of `pvq_extension::Dispatchable` and `pvq_extension::ExtensionId` for it, and
/// - helper functions like `extension_id()` and `metadata()` used by `#[extensions_impl]`.
#[proc_macro_attribute]
pub fn extension_decl(attr: TokenStream, item: TokenStream) -> TokenStream {
    extension_decl::extension_decl(attr, item)
}

/// Implement one or more previously declared extensions and generate dispatch + metadata.
///
/// Attach `#[extensions_impl]` to a `mod` that contains:
/// - one `#[extensions_impl::impl_struct]` struct (the host implementation type), and
/// - one or more `#[extensions_impl::extension] impl Trait for ImplStruct` blocks.
///
/// The macro generates:
/// - `pub type Extensions = (...)`, a tuple of macro-generated `Functions<ImplStruct>` call types,
/// - `pub fn metadata() -> pvq_extension::metadata::Metadata` for all extensions in the module.
#[proc_macro_attribute]
pub fn extensions_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    extensions_impl::extensions_impl(attr, item)
}
