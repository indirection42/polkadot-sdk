use sp_runtime::BoundedVec;
use xcm::latest::prelude::*;

pub trait ExecuteXcq<S> {
	fn execute(query: BoundedVec<u8, S>) -> (Vec<u8>, Option<Weight>);
}
