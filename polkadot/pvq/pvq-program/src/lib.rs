//! Declare PVQ (PolkaVM Query) guest programs.
//!
//! This crate is the **public entrypoint** for the PVQ program authoring macros. It re-exports
//! [`program`](macro@program), which transforms an **inline module** into a PolkaVM guest program
//! interface:
//!
//! - `#[pvq_program::program]` on an inline `mod` to declare a PVQ program.
//! - `#[program::extension_fn(...)]` to declare host extension calls.
//! - `#[program::entrypoint]` to mark functions that become guest entrypoints.
//!
//! ### Minimal example
//!
//! The following is a sketch of the expected structure. It is marked `ignore` because the macro
//! expands into PolkaVM-specific glue code and is not meant to be compiled as a standalone doctest.
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//!
//! #[pvq_program::program]
//! mod my_program {
//!     type AssetId = u32;
//!     type Balance = u64;
//!
//!     // Replace `extension_id` and `fn_index` with values that match your runtime.
//!     #[program::extension_fn(extension_id = 123u64, fn_index = 0u32)]
//!     fn total_supply(asset: AssetId) -> Balance {
//!         // The macro expands this into a host call; the body is a placeholder.
//!         unimplemented!()
//!     }
//!
//!     #[program::entrypoint]
//!     fn query(asset: AssetId) -> Balance {
//!         total_supply(asset)
//!     }
//! }
//! ```
//!
//! ### Type and encoding requirements
//!
//! All entrypoint arguments/return values and extension-call arguments/return values are expected
//! to be SCALE encodable/decodable. For custom types, derive
//! `parity_scale_codec::{Encode, Decode}` (and optionally `scale_info::TypeInfo` for richer
//! metadata).
#![cfg_attr(not(feature = "std"), no_std)]
pub use pvq_program_procedural::program;
