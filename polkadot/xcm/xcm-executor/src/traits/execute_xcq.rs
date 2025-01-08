use sp_runtime::BoundedVec;
use xcm::latest::prelude::*;

pub trait ExecuteXcq {
	type XcqSizeLimit: Get<u32>;
	fn execute(
		query: BoundedVec<u8, Self::XcqSizeLimit>,
		max_weight: Weight,
	) -> Result<(Vec<u8>, Option<Weight>), XcmError>;
}
