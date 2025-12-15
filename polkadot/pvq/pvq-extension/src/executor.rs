//! This module defines the executor for extensions.
use pvq_executor::PvqExecutor;
use pvq_primitives::{PvqError, PvqResult};

use crate::{
    perm_controller::{InvokeSource, PermissionController},
    CallDataTuple, Context,
};

/// The executor for extensions.
///
/// This struct provides an executor for running extension code.
/// It wraps a `PvqExecutor` with a `Context` for extensions.
pub struct ExtensionsExecutor<C: CallDataTuple, P: PermissionController> {
    /// The underlying PVQ executor.
    executor: PvqExecutor<Context<C, P>>,
}

impl<C: CallDataTuple, P: PermissionController> ExtensionsExecutor<C, P> {
    /// Creates a new extensions executor.
    ///
    /// `source` is forwarded to the [`PermissionController`] when handling `host_call`.
    pub fn new(source: InvokeSource) -> Self {
        let context = Context::<C, P>::new(source);
        let executor = PvqExecutor::new(Default::default(), context);
        Self { executor }
    }

    /// Executes a program with the given arguments and gas limit.
    ///
    /// - `program`: PVQ bytecode/program.
    /// - `args`: raw argument bytes passed through to the PVQ program.
    /// - `gas_limit`: optional gas limit (PVQ rules).
    ///
    /// # Returns
    ///
    /// A tuple containing the PVQ result and the remaining gas (if gas metering is enabled).
    pub fn execute(
        &mut self,
        program: &[u8],
        args: &[u8],
        gas_limit: Option<i64>,
    ) -> (PvqResult, Option<i64>) {
        let (result, gas_remaining) = self.executor.execute(program, args, gas_limit);
        tracing::info!("result: {:?}", result);
        (result.map_err(PvqError::from), gas_remaining)
    }
}
