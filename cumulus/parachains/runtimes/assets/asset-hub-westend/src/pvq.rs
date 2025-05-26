use pvq_extension::{extensions_impl, metadata::Metadata, ExtensionsExecutor, InvokeSource};

#[extensions_impl]
pub mod extensions {
	use codec::Encode;
	#[extensions_impl::impl_struct]
	pub struct ExtensionImpl;

	#[extensions_impl::extension]
	impl pvq_extension_swap::extension::ExtensionSwap for ExtensionImpl {
		type AssetId = crate::Vec<u8>;
		type Balance = crate::Balance;
		fn quote_price_tokens_for_exact_tokens(
			asset1: Self::AssetId,
			asset2: Self::AssetId,
			amount: Self::Balance,
			include_fee: bool,
		) -> Option<Self::Balance> {
			if let Ok(asset1) = codec::Decode::decode(&mut &asset1[..]) {
				if let Ok(asset2) = codec::Decode::decode(&mut &asset2[..]) {
					return crate::AssetConversion::quote_price_tokens_for_exact_tokens(
						asset1,
						asset2,
						amount,
						include_fee,
					);
				}
			}
			None
		}

		fn quote_price_exact_tokens_for_tokens(
			asset1: Self::AssetId,
			asset2: Self::AssetId,
			amount: Self::Balance,
			include_fee: bool,
		) -> Option<Self::Balance> {
			if let Ok(asset1) = codec::Decode::decode(&mut &asset1[..]) {
				if let Ok(asset2) = codec::Decode::decode(&mut &asset2[..]) {
					return crate::AssetConversion::quote_price_exact_tokens_for_tokens(
						asset1,
						asset2,
						amount,
						include_fee,
					);
				}
			}
			None
		}

		fn get_liquidity_pool(
			asset1: Self::AssetId,
			asset2: Self::AssetId,
		) -> Option<(Self::Balance, Self::Balance)> {
			if let Ok(asset1) = codec::Decode::decode(&mut &asset1[..]) {
				if let Ok(asset2) = codec::Decode::decode(&mut &asset2[..]) {
					return crate::AssetConversion::get_reserves(asset1, asset2).ok();
				}
			}
			None
		}

		fn list_pools() -> scale_info::prelude::vec::Vec<(
			Self::AssetId,
			Self::AssetId,
			Self::Balance,
			Self::Balance,
		)> {
			let pools = pallet_asset_conversion::Pools::<crate::Runtime>::iter_keys()
				.map(|pool_id| {
					let (asset1, asset2) = pool_id;
					let (balance1, balance2) =
						crate::AssetConversion::get_reserves(asset1.clone(), asset2.clone())
							.expect("Pool should exist");
					(asset1.encode(), asset2.encode(), balance1, balance2)
				})
				.collect();
			pools
		}
	}
}

pub fn execute_query(program: &[u8], args: &[u8], gas_limit: i64) -> pvq_primitives::PvqResult {
	let mut executor =
		ExtensionsExecutor::<extensions::Extensions, ()>::new(InvokeSource::RuntimeAPI);
	let (result, _) = executor.execute(program, args, Some(gas_limit));
	result
}

pub fn metadata() -> Metadata {
	extensions::metadata()
}
