use sp_runtime::BoundedVec;
use xcm::latest::prelude::*;

pub trait ExecuteXcq<S> {
	fn execute(
		query: BoundedVec<u8, S>,
		max_weight: Weight,
	) -> Result<(Vec<u8>, Option<Weight>), XcmError>;
}
