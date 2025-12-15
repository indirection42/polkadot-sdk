//! Implementation crate for the PVQ program macro.
//!
//! This crate defines the procedural macro behind [`pvq_program::program`]. Most users should
//! depend on `pvq-program` and use `#[pvq_program::program]` rather than depending on this crate
//! directly.
//!
//! ### Usage (via `pvq-program`)
//!
//! ```ignore
//! #[pvq_program::program]
//! mod query_fungibles {
//!     // Types used by the program; they must match the runtime-side implementation.
//!     type AssetId = u32;
//!     type AccountId = [u8; 32];
//!     type Balance = u64;
//!
//!     #[program::extension_fn(extension_id = 123_456u64, fn_index = 1u32)]
//!     fn balance(asset: AssetId, who: AccountId) -> Balance {
//!         // The macro expands this into a host call; the body is a placeholder.
//!         unimplemented!()
//!     }
//!
//!     #[program::entrypoint]
//!     fn sum_balance(accounts: Vec<AccountId>) -> Balance {
//!         let mut sum: Balance = 0;
//!         for account in accounts {
//!             sum += balance(0, account);
//!         }
//!         sum
//!     }
//! }
//! ```
mod program;
use proc_macro::TokenStream;

/// A procedural macro for creating PVQ programs.
#[proc_macro_attribute]
pub fn program(attr: TokenStream, item: TokenStream) -> TokenStream {
	program::program(attr, item)
}
