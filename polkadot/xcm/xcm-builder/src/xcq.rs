use xcm::latest::prelude::*;
use xcm_executor::traits::ExecuteXcq;
use xcq_extension::ExtensionsExecutor;

pub struct XcqExecutorWithoutWeightInfo<E, P>(ExtensionsExecutor<E, P>);

impl<E, P> ExecuteXcq for XcqExecutorWithoutWeightInfo<E, P> {
	fn execute(query: BoundedVec<u8, S>) -> (Vec<u8>, Option<Weight>) {
		// Encode XcqResult to Vec<u8>
		let query_result = self.0.execute_method(&query).encode();
		(query_result, None)
	}
}
