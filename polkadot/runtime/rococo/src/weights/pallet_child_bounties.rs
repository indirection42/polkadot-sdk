// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_child_bounties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-02-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `d3a9aad6f7a3`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --extrinsic=*
// --runtime=target/production/wbuild/rococo-runtime/rococo_runtime.wasm
// --pallet=pallet_child_bounties
// --header=/__w/polkadot-sdk/polkadot-sdk/polkadot/file_header.txt
// --output=./polkadot/runtime/rococo/src/weights
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --heap-pages=4096
// --no-storage-info
// --no-min-squares
// --no-median-slopes

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_child_bounties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_child_bounties::WeightInfo for WeightInfo<T> {
	/// Storage: `ChildBounties::ParentChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::Bounties` (r:1 w:0)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ParentTotalChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ParentTotalChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildBountyDescriptionsV1` (r:0 w:1)
	/// Proof: `ChildBounties::ChildBountyDescriptionsV1` (`max_values`: None, `max_size`: Some(16412), added: 18887, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildBounties` (r:0 w:1)
	/// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 16384]`.
	fn add_child_bounty(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `540`
		//  Estimated: `6196`
		// Minimum execution time: 69_990_000 picoseconds.
		Weight::from_parts(72_661_004, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			// Standard Error: 12
			.saturating_add(Weight::from_parts(589, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:0)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildrenCuratorFees` (r:1 w:1)
	/// Proof: `ChildBounties::ChildrenCuratorFees` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn propose_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `604`
		//  Estimated: `3642`
		// Minimum execution time: 19_875_000 picoseconds.
		Weight::from_parts(20_672_000, 0)
			.saturating_add(Weight::from_parts(0, 3642))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:0)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn accept_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `773`
		//  Estimated: `3642`
		// Minimum execution time: 35_098_000 picoseconds.
		Weight::from_parts(36_400_000, 0)
			.saturating_add(Weight::from_parts(0, 3642))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::Bounties` (r:1 w:0)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn unassign_curator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `773`
		//  Estimated: `3642`
		// Minimum execution time: 46_447_000 picoseconds.
		Weight::from_parts(47_620_000, 0)
			.saturating_add(Weight::from_parts(0, 3642))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:0)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	fn award_child_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `670`
		//  Estimated: `3642`
		// Minimum execution time: 22_125_000 picoseconds.
		Weight::from_parts(22_974_000, 0)
			.saturating_add(Weight::from_parts(0, 3642))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ParentChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildBountyDescriptionsV1` (r:0 w:1)
	/// Proof: `ChildBounties::ChildBountyDescriptionsV1` (`max_values`: None, `max_size`: Some(16412), added: 18887, mode: `MaxEncodedLen`)
	fn claim_child_bounty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `609`
		//  Estimated: `8799`
		// Minimum execution time: 110_570_000 picoseconds.
		Weight::from_parts(112_195_000, 0)
			.saturating_add(Weight::from_parts(0, 8799))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:0)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildrenCuratorFees` (r:1 w:1)
	/// Proof: `ChildBounties::ChildrenCuratorFees` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ParentChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildBountyDescriptionsV1` (r:0 w:1)
	/// Proof: `ChildBounties::ChildBountyDescriptionsV1` (`max_values`: None, `max_size`: Some(16412), added: 18887, mode: `MaxEncodedLen`)
	fn close_child_bounty_added() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `850`
		//  Estimated: `6196`
		// Minimum execution time: 76_645_000 picoseconds.
		Weight::from_parts(78_896_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Bounties::Bounties` (r:1 w:0)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildrenCuratorFees` (r:1 w:1)
	/// Proof: `ChildBounties::ChildrenCuratorFees` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ParentChildBounties` (r:1 w:1)
	/// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `ChildBounties::ChildBountyDescriptionsV1` (r:0 w:1)
	/// Proof: `ChildBounties::ChildBountyDescriptionsV1` (`max_values`: None, `max_size`: Some(16412), added: 18887, mode: `MaxEncodedLen`)
	fn close_child_bounty_active() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1034`
		//  Estimated: `8799`
		// Minimum execution time: 94_008_000 picoseconds.
		Weight::from_parts(95_861_000, 0)
			.saturating_add(Weight::from_parts(0, 8799))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}
