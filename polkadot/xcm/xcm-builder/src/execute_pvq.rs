use alloc::vec::Vec;
use codec::{Decode, Encode};
use pvq_extension::{
	extensions_impl, CallDataTuple, ExtensionsExecutor, InvokeSource, PermissionController,
};
use pvq_primitives::PvqError;
use xcm::prelude::*;
use xcm_executor::traits::{ExecutePvq, GasWeightConverter};

pub struct ExecutorWithoutRefund<E: CallDataTuple, P: PermissionController, G: GasWeightConverter> {
	executor: ExtensionsExecutor<E, P>,
	_phantom: core::marker::PhantomData<G>,
}

pub struct TestGasWeightConverter;

impl GasWeightConverter for TestGasWeightConverter {
	fn weight_to_gas(_weight: Weight) -> i64 {
		1000_000_000
	}
	fn gas_to_weight(_gas: i64) -> Weight {
		Weight::from_parts(1000_000_000 as u64, 1000)
	}
}

#[extensions_impl]
pub mod extensions {
	use codec::Decode;
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

pub type TestPvqExecutor =
	ExecutorWithoutRefund<extensions::Extensions, (), TestGasWeightConverter>;

impl<E: CallDataTuple, P: PermissionController, C: GasWeightConverter> ExecutePvq
	for ExecutorWithoutRefund<E, P, C>
{
	fn new() -> Self {
		let executor = ExtensionsExecutor::new(InvokeSource::XCM);
		Self { executor, _phantom: core::marker::PhantomData }
	}

	fn execute(
		&mut self,
		query: Vec<u8>,
		max_weight: Weight,
	) -> (Result<Vec<u8>, XcmError>, Option<Weight>) {
		let (program, args): (Vec<u8>, Vec<u8>) = match Decode::decode(&mut &query[..]) {
			Ok(result) => result,
			Err(_) => return (Err(XcmError::FailedToDecode), None),
		};
		let (result, _) =
			self.executor.execute(&program, &args, Some(C::weight_to_gas(max_weight)));
		let result = match result {
			Ok(result) => Ok(result),
			Err(e) => match e {
				PvqError::FailedToDecode => Err(XcmError::FailedToDecode),
				PvqError::InvalidPvqProgramFormat => Err(XcmError::FailedToDecode),
				PvqError::QueryExceedsWeightLimit => Err(XcmError::MaxWeightInvalid),
				PvqError::Trap => Ok(PvqError::Trap.encode()),
				PvqError::MemoryAccessError => Ok(PvqError::MemoryAccessError.encode()),
				PvqError::HostCallError => Ok(PvqError::HostCallError.encode()),
				PvqError::Other => Ok(PvqError::Other.encode()),
			},
		};
		// Without refunding weight
		(result, None)
	}
}
