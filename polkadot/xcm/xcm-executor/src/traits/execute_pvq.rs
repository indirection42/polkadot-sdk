use xcm::latest::prelude::*;

pub trait ExecutePvq {
	fn new() -> Self;
	fn execute(
		&mut self,
		query: Vec<u8>,
		max_weight: Weight,
	) -> (Result<Vec<u8>, XcmError>, Option<Weight>);
}

pub trait GasWeightConverter {
	fn weight_to_gas(weight: Weight) -> i64;
	fn gas_to_weight(gas: i64) -> Weight;
}
