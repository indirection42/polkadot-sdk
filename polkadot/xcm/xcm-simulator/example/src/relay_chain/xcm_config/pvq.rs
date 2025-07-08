use pvq_extension::extensions_impl;
#[extensions_impl]
pub mod extensions {
	#[extensions_impl::impl_struct]
	pub struct ExtensionsImpl;

	#[extensions_impl::extension]
	impl pvq_extension_core::extension::ExtensionCore for ExtensionsImpl {
		type ExtensionId = u64;
		fn has_extension(id: Self::ExtensionId) -> bool {
			matches!(id, 0 | 1)
		}
	}

	#[extensions_impl::extension]
	impl pvq_extension_fungibles::extension::ExtensionFungibles for ExtensionsImpl {
		type AssetId = u32;
		type AccountId = [u8; 32];
		type Balance = u64;
		fn total_supply(_asset: Self::AssetId) -> Self::Balance {
			100
		}
		fn balance(_asset: Self::AssetId, _who: Self::AccountId) -> Self::Balance {
			100
		}
	}
}
