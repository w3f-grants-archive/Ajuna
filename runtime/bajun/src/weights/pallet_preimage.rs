// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_preimage`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `eric-macos.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/bajun-para
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-preimage
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-AGPL
// --output=./runtime/bajun/src/weights/pallet_preimage.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_preimage`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_preimage::WeightInfo for WeightInfo<T> {
	// Storage: PreImage StatusFor (r:1 w:1)
	// Storage: PreImage PreimageFor (r:0 w:1)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_preimage(s: u32, ) -> Weight {
		// Minimum execution time: 28_000 nanoseconds.
		Weight::from_ref_time(29_000_000 as u64)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(1_407 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PreImage StatusFor (r:1 w:1)
	// Storage: PreImage PreimageFor (r:0 w:1)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_requested_preimage(s: u32, ) -> Weight {
		// Minimum execution time: 19_000 nanoseconds.
		Weight::from_ref_time(19_000_000 as u64)
			// Standard Error: 2
			.saturating_add(Weight::from_ref_time(1_402 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PreImage StatusFor (r:1 w:1)
	// Storage: PreImage PreimageFor (r:0 w:1)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_no_deposit_preimage(s: u32, ) -> Weight {
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(18_000_000 as u64)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(1_399 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PreImage StatusFor (r:1 w:1)
	// Storage: PreImage PreimageFor (r:0 w:1)
	fn unnote_preimage() -> Weight {
		// Minimum execution time: 41_000 nanoseconds.
		Weight::from_ref_time(52_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PreImage StatusFor (r:1 w:1)
	// Storage: PreImage PreimageFor (r:0 w:1)
	fn unnote_no_deposit_preimage() -> Weight {
		// Minimum execution time: 33_000 nanoseconds.
		Weight::from_ref_time(39_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PreImage StatusFor (r:1 w:1)
	fn request_preimage() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(36_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PreImage StatusFor (r:1 w:1)
	fn request_no_deposit_preimage() -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(23_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PreImage StatusFor (r:1 w:1)
	fn request_unnoted_preimage() -> Weight {
		// Minimum execution time: 16_000 nanoseconds.
		Weight::from_ref_time(17_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PreImage StatusFor (r:1 w:1)
	fn request_requested_preimage() -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(9_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PreImage StatusFor (r:1 w:1)
	// Storage: PreImage PreimageFor (r:0 w:1)
	fn unrequest_preimage() -> Weight {
		// Minimum execution time: 33_000 nanoseconds.
		Weight::from_ref_time(41_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PreImage StatusFor (r:1 w:1)
	fn unrequest_unnoted_preimage() -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PreImage StatusFor (r:1 w:1)
	fn unrequest_multi_referenced_preimage() -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(9_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
