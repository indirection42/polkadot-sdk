#![cfg_attr(not(feature = "std"), no_std)]
//! # PVQ Extension System
//!
//! This crate provides the *host-side* extension system for PVQ (PolkaVM Query).
//! It allows defining and implementing extensions that can be called from PVQ programs via the
//! `host_call` host function exposed by the [`pvq_executor`].
//!
//! ## Overview
//!
//! The extension system consists of:
//!
//! - **Extension definitions**: traits that define the API of an extension.
//! - **Extension implementations**: implementations of those traits for your runtime.
//! - **Execution**: [`ExtensionsExecutor`] runs a PVQ program and wires up `host_call` so the PVQ
//!   program can invoke your extensions.
//! - **Permissions**: [`PermissionController`] decides whether a given invocation is allowed.
//!
//! ## Usage
//!
//! Extensions are defined using the `#[extension_decl]` macro and implemented using the
//! `#[extensions_impl]` macro (re-exported from `pvq-extension-procedural`).
//!
//! The `#[extensions_impl]` module generates:
//! - a [`CallDataTuple`] type alias named `Extensions`, used to dispatch calls at runtime, and
//! - a `metadata()` function returning [`metadata::Metadata`] describing the exposed extensions.
//!
//! For a concrete integration example in this repository, see
//! `cumulus/parachains/runtimes/assets/asset-hub-westend/src/pvq.rs`.
//!
//! ### Minimal wiring (host side)
//!
//! The snippet below shows how to define an extension, implement it, and construct an executor.
//! It is intentionally `no_run` since running requires PVQ program bytes.
//!
//! ```no_run
//! use pvq_extension::{extension_decl, extensions_impl, ExtensionsExecutor, InvokeSource};
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
//!         fn add(a: u32, b: u32) -> u32 {
//!             a.saturating_add(b)
//!         }
//!     }
//! }
//!
//! // Construct an executor using the generated `extensions::Extensions` dispatcher and the default
//! // permission controller `()`, which allows all calls.
//! let mut exec = ExtensionsExecutor::<extensions::Extensions, ()>::new(InvokeSource::RuntimeAPI);
//! let _meta = extensions::metadata();
//! # let _ = &mut exec;
//! ```

// Re-exports
pub use pvq_extension_procedural::{extension_decl, extensions_impl};

// Module declarations
mod calldata;
mod context;
mod error;
mod executor;
mod macros;
pub mod metadata;
mod perm_controller;

// Public exports
pub use calldata::{CallData, DispatchError, Dispatchable, ExtensionId, ExtensionIdTy};
pub use context::Context;
pub use error::ExtensionError;
pub use executor::ExtensionsExecutor;
pub use macros::CallDataTuple;
pub use metadata::{ExtensionImplMetadata, ExtensionMetadata};
pub use perm_controller::{InvokeSource, PermissionController};
