//! # PVQ Swap Extension
//!
//! This crate declares a PVQ (PolkaVM Query) extension for retrieving **swap-related, read-only**
//! information (quotes and liquidity pool data).
//!
//! ## What this crate provides
//!
//! - A single extension trait: [`extension::ExtensionSwap`]
//! - A stable extension interface that can be implemented by a runtime and invoked by PVQ guests
//!
//! This crate only defines the interface. How quotes/pools are sourced (e.g. which AMM/DEX pallet,
//! routing, fee model) is up to the runtime implementation.
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use pvq_extension::extension_decl;

/// The swap PVQ extension declaration.
///
/// The `#[extension_decl]` macro expands this module into a PVQ extension interface and metadata
/// that can be wired into an [`pvq_extension::ExtensionsExecutor`].
#[extension_decl]
pub mod extension {
    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;

    /// The swap PVQ extension trait.
    ///
    /// Implement this trait in the runtime to expose swap quotes and pool/asset information to PVQ
    /// programs.
    ///
    /// All functions are expected to be **read-only**. `None` typically indicates that the quote
    /// or pool information is not available (missing pool, no route, unsupported pair, etc.).
    #[extension_decl::extension]
    pub trait ExtensionSwap {
        /// The asset identifier type.
        type AssetId;
        /// The balance type.
        type Balance;
        /// The asset info type.
        type AssetInfo;

        /// Quote how much `asset1` you would receive for an exact `amount` of `asset2`.
        ///
        /// # Arguments
        ///
        /// * `asset1`: The asset to be received (output).
        /// * `asset2`: The asset to be paid (input).
        /// * `amount`: The input amount of `asset2`.
        /// * `include_fee`: Whether the returned output amount accounts for fees.
        ///
        /// # Returns
        ///
        /// The quoted output amount of `asset1`, or `None` if no quote is available.
        fn quote_price_tokens_for_exact_tokens(
            asset1: Self::AssetId,
            asset2: Self::AssetId,
            amount: Self::Balance,
            include_fee: bool,
        ) -> Option<Self::Balance>;

        /// Quote how much `asset1` you would need to pay to receive an exact `amount` of `asset2`.
        ///
        /// # Arguments
        ///
        /// * `asset1`: The asset to be paid (input).
        /// * `asset2`: The asset to be received (output).
        /// * `amount`: The desired output amount of `asset2`.
        /// * `include_fee`: Whether the returned input amount accounts for fees.
        ///
        /// # Returns
        ///
        /// The quoted required input amount of `asset1`, or `None` if no quote is available.
        fn quote_price_exact_tokens_for_tokens(
            asset1: Self::AssetId,
            asset2: Self::AssetId,
            amount: Self::Balance,
            include_fee: bool,
        ) -> Option<Self::Balance>;

        /// Get the liquidity pool of two assets.
        ///
        /// # Arguments
        ///
        /// * `asset1`: The identifier of the first asset.
        /// * `asset2`: The identifier of the second asset.
        ///
        /// # Returns
        ///
        /// A tuple containing the balances for the asset pair in the pool, or `None` if the pool
        /// does not exist.
        fn get_liquidity_pool(
            asset1: Self::AssetId,
            asset2: Self::AssetId,
        ) -> Option<(Self::Balance, Self::Balance)>;

        /// List all available liquidity pools.
        ///
        /// # Returns
        ///
        /// A list of tuples, where each tuple represents a liquidity pool and contains the
        /// identifiers of the two assets in the pool.
        fn list_pools() -> Vec<(Self::AssetId, Self::AssetId)>;

        /// Get information about a specific asset.
        ///
        /// # Arguments
        ///
        /// * `asset`: The identifier of the asset.
        ///
        /// # Returns
        ///
        /// Information about the asset, or `None` if the asset is unknown.
        fn asset_info(asset: Self::AssetId) -> Option<Self::AssetInfo>;

        /// Get information about all assets.
        ///
        /// # Returns
        ///
        /// A map of asset identifiers to asset information.
        ///
        /// Implementations may choose to return a subset of assets, or may treat this call as
        /// potentially expensive.
        fn assets_info() -> BTreeMap<Self::AssetId, Self::AssetInfo>;
    }
}
