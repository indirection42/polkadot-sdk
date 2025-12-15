#![cfg_attr(not(feature = "std"), no_std)]
//! # PVQ extension fungibles
//!
//! Defines an example fungibles PVQ extension.
//!
//! ## Overview
//!
//! This extension provides a small, read-only API for querying fungible asset metadata and balances
//! from PVQ programs.
//!
//! ## Usage
//!
//! PVQ programs typically import the trait and call the generated host functions directly:
//!
//! ```ignore
//! use pvq_extension_fungibles::extension::ExtensionFungibles;
//!
//! // Concrete types (e.g. `AssetId`, `AccountId`, `Balance`) are provided by the host/runtime.
//! // let balance = ExtensionFungibles::balance(asset_id, account_id);
//! ```
//!
//! ## Feature flags
//!
//! - `std`: Enables standard library support.
use pvq_extension::extension_decl;

/// Fungibles extension definitions for PVQ programs.
#[extension_decl]
pub mod extension {
	//! Fungibles extension definition for PVQ programs.
	//!
	//! This module is augmented by the [`pvq_extension::extension_decl`] macro.

	use scale_info::prelude::vec::Vec;

	/// The fungibles PVQ extension trait.
	#[extension_decl::extension]
	pub trait ExtensionFungibles {
		/// Identifier of an asset.
		type AssetId;
		/// A fungible balance.
		type Balance;
		/// An account identifier.
		type AccountId;

		/// Check if an asset exists.
		///
		/// Returns `true` if `asset` exists, `false` otherwise.
		fn asset_exists(asset: Self::AssetId) -> bool;

		/// Get the name of an asset.
		///
		/// The returned value is an opaque byte string (typically UTF-8).
		///
		/// Callers that need to distinguish between "unknown asset" and an empty name should first
		/// call [`Self::asset_exists`].
		fn name(asset: Self::AssetId) -> Vec<u8>;

		/// Get the symbol of an asset.
		///
		/// The returned value is an opaque byte string (typically UTF-8).
		///
		/// Callers that need to distinguish between "unknown asset" and an empty symbol should
		/// first call [`Self::asset_exists`].
		fn symbol(asset: Self::AssetId) -> Vec<u8>;

		/// Get the decimals of an asset.
		///
		/// Callers that need to distinguish between "unknown asset" and a `0` value should first
		/// call [`Self::asset_exists`].
		fn decimals(asset: Self::AssetId) -> u8;

		/// Get the minimum balance of an asset.
		///
		/// Callers that need to distinguish between "unknown asset" and a `0` value should first
		/// call [`Self::asset_exists`].
		fn minimum_balance(asset: Self::AssetId) -> Self::Balance;

		/// Get the total supply of an asset.
		///
		/// Callers that need to distinguish between "unknown asset" and a `0` value should first
		/// call [`Self::asset_exists`].
		fn total_supply(asset: Self::AssetId) -> Self::Balance;

		/// Get the balance of an asset for a specific account.
		///
		/// Callers that need to distinguish between "unknown asset" and a `0` value should first
		/// call [`Self::asset_exists`].
		fn balance(asset: Self::AssetId, who: Self::AccountId) -> Self::Balance;
	}
}
