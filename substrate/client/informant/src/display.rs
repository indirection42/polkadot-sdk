// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use console::style;
use log::info;
use sc_client_api::ClientInfo;
use sc_network::NetworkStatus;
use sc_network_sync::{SyncState, SyncStatus, WarpSyncPhase, WarpSyncProgress};
use sp_runtime::traits::{Block as BlockT, CheckedDiv, NumberFor, Saturating, Zero};
use std::{fmt, time::Instant};

use crate::PrintFullHashOnDebugLogging;

/// State of the informant display system.
///
/// This is the system that handles the line that gets regularly printed and that looks something
/// like:
///
/// > Syncing  5.4 bps, target=#531028 (4 peers), best: #90683 (0x4ca8…51b8),
/// > finalized #360 (0x6f24…a38b), ⬇ 5.5kiB/s ⬆ 0.9kiB/s
///
/// # Usage
///
/// Call `InformantDisplay::new` to initialize the state, then regularly call `display` with the
/// information to display.
pub struct InformantDisplay<B: BlockT> {
	/// Head of chain block number from the last time `display` has been called.
	/// `None` if `display` has never been called.
	last_number: Option<NumberFor<B>>,
	/// The last time `display` or `new` has been called.
	last_update: Instant,
	/// The last seen total of bytes received.
	last_total_bytes_inbound: u64,
	/// The last seen total of bytes sent.
	last_total_bytes_outbound: u64,
}

impl<B: BlockT> InformantDisplay<B> {
	/// Builds a new informant display system.
	pub fn new() -> InformantDisplay<B> {
		InformantDisplay {
			last_number: None,
			last_update: Instant::now(),
			last_total_bytes_inbound: 0,
			last_total_bytes_outbound: 0,
		}
	}

	/// Displays the informant by calling `info!`.
	pub fn display(
		&mut self,
		info: &ClientInfo<B>,
		net_status: NetworkStatus,
		sync_status: SyncStatus<B>,
		num_connected_peers: usize,
	) {
		let best_number = info.chain.best_number;
		let best_hash = info.chain.best_hash;
		let finalized_number = info.chain.finalized_number;
		let speed = speed::<B>(best_number, self.last_number, self.last_update);
		let total_bytes_inbound = net_status.total_bytes_inbound;
		let total_bytes_outbound = net_status.total_bytes_outbound;

		let now = Instant::now();
		let elapsed = (now - self.last_update).as_secs();
		self.last_update = now;
		self.last_number = Some(best_number);

		let diff_bytes_inbound = total_bytes_inbound - self.last_total_bytes_inbound;
		let diff_bytes_outbound = total_bytes_outbound - self.last_total_bytes_outbound;
		let (avg_bytes_per_sec_inbound, avg_bytes_per_sec_outbound) = if elapsed > 0 {
			self.last_total_bytes_inbound = total_bytes_inbound;
			self.last_total_bytes_outbound = total_bytes_outbound;
			(diff_bytes_inbound / elapsed, diff_bytes_outbound / elapsed)
		} else {
			(diff_bytes_inbound, diff_bytes_outbound)
		};

		let (level, status, target) =
			match (sync_status.state, sync_status.state_sync, sync_status.warp_sync) {
				// Do not set status to "Block history" when we are doing a major sync.
				//
				// A node could for example have been warp synced to the tip of the chain and
				// shutdown. At the next start we still need to download the block history, but
				// first will sync to the tip of the chain.
				(
					sync_status,
					_,
					Some(WarpSyncProgress { phase: WarpSyncPhase::DownloadingBlocks(n), .. }),
				) if !sync_status.is_major_syncing() => ("⏩", "Block history".into(), format!(", #{}", n)),
				// Handle all phases besides the two phases we already handle above.
				(_, _, Some(warp))
					if !matches!(warp.phase, WarpSyncPhase::DownloadingBlocks(_)) =>
					(
						"⏩",
						"Warping".into(),
						format!(
							", {}, {:.2} Mib",
							warp.phase,
							(warp.total_bytes as f32) / (1024f32 * 1024f32)
						),
					),
				(_, Some(state), _) => (
					"⚙️ ",
					"State sync".into(),
					format!(
						", {}, {}%, {:.2} Mib",
						state.phase,
						state.percentage,
						(state.size as f32) / (1024f32 * 1024f32)
					),
				),
				(SyncState::Idle, _, _) => ("💤", "Idle".into(), "".into()),
				(SyncState::Downloading { target }, _, _) =>
					("⚙️ ", format!("Syncing{}", speed), format!(", target=#{target}")),
				(SyncState::Importing { target }, _, _) =>
					("⚙️ ", format!("Preparing{}", speed), format!(", target=#{target}")),
			};

		info!(
			target: "substrate",
			"{} {}{} ({} peers), best: #{} ({}), finalized #{} ({}), ⬇ {} ⬆ {}",
			level,
			style(&status).white().bold(),
			target,
			style(num_connected_peers).white().bold(),
			style(best_number).white().bold(),
			PrintFullHashOnDebugLogging(&best_hash),
			style(finalized_number).white().bold(),
			PrintFullHashOnDebugLogging(&info.chain.finalized_hash),
			style(TransferRateFormat(avg_bytes_per_sec_inbound)).green(),
			style(TransferRateFormat(avg_bytes_per_sec_outbound)).red(),
		)
	}
}

/// Calculates `(best_number - last_number) / (now - last_update)` and returns a `String`
/// representing the speed of import.
fn speed<B: BlockT>(
	best_number: NumberFor<B>,
	last_number: Option<NumberFor<B>>,
	last_update: Instant,
) -> String {
	// Number of milliseconds elapsed since last time.
	let elapsed_ms = {
		let elapsed = last_update.elapsed();
		let since_last_millis = elapsed.as_secs() * 1000;
		let since_last_subsec_millis = elapsed.subsec_millis() as u64;
		since_last_millis + since_last_subsec_millis
	};

	// Number of blocks that have been imported since last time.
	let diff = match last_number {
		None => return String::new(),
		Some(n) => best_number.saturating_sub(n),
	};

	if let Ok(diff) = TryInto::<u128>::try_into(diff) {
		// If the number of blocks can be converted to a regular integer, then it's easy: just
		// do the math and turn it into a `f64`.
		let speed = diff
			.saturating_mul(10_000)
			.checked_div(u128::from(elapsed_ms))
			.map_or(0.0, |s| s as f64) /
			10.0;
		format!(" {:4.1} bps", speed)
	} else {
		// If the number of blocks can't be converted to a regular integer, then we need a more
		// algebraic approach and we stay within the realm of integers.
		let one_thousand = NumberFor::<B>::from(1_000u32);
		let elapsed =
			NumberFor::<B>::from(<u32 as TryFrom<_>>::try_from(elapsed_ms).unwrap_or(u32::MAX));

		let speed = diff
			.saturating_mul(one_thousand)
			.checked_div(&elapsed)
			.unwrap_or_else(Zero::zero);
		format!(" {} bps", speed)
	}
}

/// Contains a number of bytes per second. Implements `fmt::Display` and shows this number of bytes
/// per second in a nice way.
struct TransferRateFormat(u64);
impl fmt::Display for TransferRateFormat {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Special case 0.
		if self.0 == 0 {
			return write!(f, "0")
		}

		// Under 0.1 kiB, display plain bytes.
		if self.0 < 100 {
			return write!(f, "{} B/s", self.0)
		}

		// Under 1.0 MiB/sec, display the value in kiB/sec.
		if self.0 < 1024 * 1024 {
			return write!(f, "{:.1}kiB/s", self.0 as f64 / 1024.0)
		}

		write!(f, "{:.1}MiB/s", self.0 as f64 / (1024.0 * 1024.0))
	}
}
