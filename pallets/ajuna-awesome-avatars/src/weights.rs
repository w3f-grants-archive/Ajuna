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

//! Autogenerated weights for pallet_ajuna_awesome_avatars
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-01, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `weight-calculation`, CPU: `DO-Regular`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/bajun-para
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-ajuna-awesome-avatars
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./.maintain/frame-weight-template.hbs
// --output=./pallets/ajuna-awesome-avatars/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_ajuna_awesome_avatars.
pub trait WeightInfo {
	fn mint_free(n: u32, ) -> Weight;
	fn mint_normal(n: u32, ) -> Weight;
	fn forge(n: u32, ) -> Weight;
	fn transfer_avatar_normal(n: u32, ) -> Weight;
	fn transfer_avatar_organizer(n: u32, ) -> Weight;
	fn transfer_free_mints() -> Weight;
	fn set_price() -> Weight;
	fn remove_price() -> Weight;
	fn buy(n: u32, ) -> Weight;
	fn upgrade_storage() -> Weight;
	fn set_organizer() -> Weight;
	fn set_treasurer() -> Weight;
	fn claim_treasury() -> Weight;
	fn set_season() -> Weight;
	fn update_global_config() -> Weight;
	fn set_free_mints() -> Weight;
}

/// Weights for pallet_ajuna_awesome_avatars using the Substrate node and recommended hardware.
pub struct AjunaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AjunaWeight<T> {
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonStatus (r:1 w:0)
	// Storage: AwesomeAvatars Accounts (r:1 w:1)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:0)
	// Storage: Randomness RandomMaterial (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: AwesomeAvatars Owners (r:1 w:1)
	// Storage: AwesomeAvatars SeasonStats (r:1 w:1)
	// Storage: AwesomeAvatars Avatars (r:0 w:6)
	/// The range of component `n` is `[0, 94]`.
	fn mint_free(n: u32, ) -> Weight {
		// Minimum execution time: 213_446 nanoseconds.
		Weight::from_ref_time(445_003_056 as u64)
			// Standard Error: 128_679
			.saturating_add(Weight::from_ref_time(603_264 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(10 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonStatus (r:1 w:0)
	// Storage: AwesomeAvatars Accounts (r:1 w:1)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:0)
	// Storage: Randomness RandomMaterial (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AwesomeAvatars Owners (r:1 w:1)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	// Storage: AwesomeAvatars SeasonStats (r:1 w:1)
	// Storage: AwesomeAvatars Avatars (r:0 w:6)
	/// The range of component `n` is `[0, 94]`.
	fn mint_normal(n: u32, ) -> Weight {
		// Minimum execution time: 249_373 nanoseconds.
		Weight::from_ref_time(535_576_523 as u64)
			// Standard Error: 200_437
			.saturating_add(Weight::from_ref_time(1_553_818 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonStatus (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:5 w:0)
	// Storage: AwesomeAvatars LockedAvatars (r:5 w:0)
	// Storage: AwesomeAvatars Avatars (r:5 w:5)
	// Storage: Randomness RandomMaterial (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: AwesomeAvatars Owners (r:1 w:1)
	// Storage: AwesomeAvatars Accounts (r:1 w:1)
	// Storage: AwesomeAvatars SeasonStats (r:1 w:1)
	/// The range of component `n` is `[5, 100]`.
	fn forge(n: u32, ) -> Weight {
		// Minimum execution time: 216_064 nanoseconds.
		Weight::from_ref_time(394_484_946 as u64)
			// Standard Error: 87_869
			.saturating_add(Weight::from_ref_time(1_202_618 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(24 as u64))
			.saturating_add(T::DbWeight::get().writes(9 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Organizer (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:1 w:0)
	// Storage: AwesomeAvatars Avatars (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	// Storage: AwesomeAvatars Owners (r:2 w:2)
	// Storage: AwesomeAvatars Accounts (r:1 w:0)
	/// The range of component `n` is `[1, 100]`.
	fn transfer_avatar_normal(_n: u32, ) -> Weight {
		// Minimum execution time: 272_006 nanoseconds.
		Weight::from_ref_time(573_822_739 as u64)
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Organizer (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:1 w:0)
	// Storage: AwesomeAvatars Avatars (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	// Storage: AwesomeAvatars Owners (r:2 w:2)
	// Storage: AwesomeAvatars Accounts (r:1 w:0)
	/// The range of component `n` is `[1, 100]`.
	fn transfer_avatar_organizer(_n: u32, ) -> Weight {
		// Minimum execution time: 244_415 nanoseconds.
		Weight::from_ref_time(549_888_990 as u64)
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Accounts (r:2 w:2)
	fn transfer_free_mints() -> Weight {
		// Minimum execution time: 51_402 nanoseconds.
		Weight::from_ref_time(116_205_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Avatars (r:1 w:0)
	// Storage: AwesomeAvatars LockedAvatars (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:0 w:1)
	fn set_price() -> Weight {
		// Minimum execution time: 113_778 nanoseconds.
		Weight::from_ref_time(182_926_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:1 w:1)
	// Storage: AwesomeAvatars Avatars (r:1 w:0)
	fn remove_price() -> Weight {
		// Minimum execution time: 104_826 nanoseconds.
		Weight::from_ref_time(179_753_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:1 w:1)
	// Storage: AwesomeAvatars Avatars (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:0)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	// Storage: AwesomeAvatars Owners (r:2 w:2)
	// Storage: AwesomeAvatars Accounts (r:2 w:2)
	/// The range of component `n` is `[1, 100]`.
	fn buy(n: u32, ) -> Weight {
		// Minimum execution time: 207_877 nanoseconds.
		Weight::from_ref_time(454_309_724 as u64)
			// Standard Error: 151_022
			.saturating_add(Weight::from_ref_time(2_517_327 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(10 as u64))
	}
	// Storage: AwesomeAvatars Accounts (r:1 w:1)
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	fn upgrade_storage() -> Weight {
		// Minimum execution time: 194_128 nanoseconds.
		Weight::from_ref_time(218_627_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: AwesomeAvatars Organizer (r:0 w:1)
	fn set_organizer() -> Weight {
		// Minimum execution time: 50_164 nanoseconds.
		Weight::from_ref_time(56_488_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars Treasurer (r:0 w:1)
	fn set_treasurer() -> Weight {
		// Minimum execution time: 55_186 nanoseconds.
		Weight::from_ref_time(61_604_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars Treasurer (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonStatus (r:1 w:0)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn claim_treasury() -> Weight {
		// Minimum execution time: 166_460 nanoseconds.
		Weight::from_ref_time(191_121_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: AwesomeAvatars Organizer (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:1)
	fn set_season() -> Weight {
		// Minimum execution time: 82_258 nanoseconds.
		Weight::from_ref_time(92_048_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars Organizer (r:1 w:0)
	// Storage: AwesomeAvatars GlobalConfigs (r:0 w:1)
	fn update_global_config() -> Weight {
		// Minimum execution time: 69_141 nanoseconds.
		Weight::from_ref_time(75_555_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars Organizer (r:1 w:0)
	// Storage: AwesomeAvatars Accounts (r:1 w:1)
	fn set_free_mints() -> Weight {
		// Minimum execution time: 69_824 nanoseconds.
		Weight::from_ref_time(76_287_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonStatus (r:1 w:0)
	// Storage: AwesomeAvatars Accounts (r:1 w:1)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:0)
	// Storage: Randomness RandomMaterial (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: AwesomeAvatars Owners (r:1 w:1)
	// Storage: AwesomeAvatars SeasonStats (r:1 w:1)
	// Storage: AwesomeAvatars Avatars (r:0 w:6)
	/// The range of component `n` is `[0, 94]`.
	fn mint_free(n: u32, ) -> Weight {
		// Minimum execution time: 213_446 nanoseconds.
		Weight::from_ref_time(445_003_056 as u64)
			// Standard Error: 128_679
			.saturating_add(Weight::from_ref_time(603_264 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().writes(10 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonStatus (r:1 w:0)
	// Storage: AwesomeAvatars Accounts (r:1 w:1)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:0)
	// Storage: Randomness RandomMaterial (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AwesomeAvatars Owners (r:1 w:1)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	// Storage: AwesomeAvatars SeasonStats (r:1 w:1)
	// Storage: AwesomeAvatars Avatars (r:0 w:6)
	/// The range of component `n` is `[0, 94]`.
	fn mint_normal(n: u32, ) -> Weight {
		// Minimum execution time: 249_373 nanoseconds.
		Weight::from_ref_time(535_576_523 as u64)
			// Standard Error: 200_437
			.saturating_add(Weight::from_ref_time(1_553_818 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(11 as u64))
			.saturating_add(RocksDbWeight::get().writes(12 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonStatus (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:5 w:0)
	// Storage: AwesomeAvatars LockedAvatars (r:5 w:0)
	// Storage: AwesomeAvatars Avatars (r:5 w:5)
	// Storage: Randomness RandomMaterial (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: AwesomeAvatars Owners (r:1 w:1)
	// Storage: AwesomeAvatars Accounts (r:1 w:1)
	// Storage: AwesomeAvatars SeasonStats (r:1 w:1)
	/// The range of component `n` is `[5, 100]`.
	fn forge(n: u32, ) -> Weight {
		// Minimum execution time: 216_064 nanoseconds.
		Weight::from_ref_time(394_484_946 as u64)
			// Standard Error: 87_869
			.saturating_add(Weight::from_ref_time(1_202_618 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(24 as u64))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Organizer (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:1 w:0)
	// Storage: AwesomeAvatars Avatars (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	// Storage: AwesomeAvatars Owners (r:2 w:2)
	// Storage: AwesomeAvatars Accounts (r:1 w:0)
	/// The range of component `n` is `[1, 100]`.
	fn transfer_avatar_normal(_n: u32, ) -> Weight {
		// Minimum execution time: 272_006 nanoseconds.
		Weight::from_ref_time(573_822_739 as u64)
			.saturating_add(RocksDbWeight::get().reads(10 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Organizer (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:1 w:0)
	// Storage: AwesomeAvatars Avatars (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	// Storage: AwesomeAvatars Owners (r:2 w:2)
	// Storage: AwesomeAvatars Accounts (r:1 w:0)
	/// The range of component `n` is `[1, 100]`.
	fn transfer_avatar_organizer(_n: u32, ) -> Weight {
		// Minimum execution time: 244_415 nanoseconds.
		Weight::from_ref_time(549_888_990 as u64)
			.saturating_add(RocksDbWeight::get().reads(10 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Accounts (r:2 w:2)
	fn transfer_free_mints() -> Weight {
		// Minimum execution time: 51_402 nanoseconds.
		Weight::from_ref_time(116_205_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Avatars (r:1 w:0)
	// Storage: AwesomeAvatars LockedAvatars (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:0 w:1)
	fn set_price() -> Weight {
		// Minimum execution time: 113_778 nanoseconds.
		Weight::from_ref_time(182_926_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:1 w:1)
	// Storage: AwesomeAvatars Avatars (r:1 w:0)
	fn remove_price() -> Weight {
		// Minimum execution time: 104_826 nanoseconds.
		Weight::from_ref_time(179_753_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: AwesomeAvatars Trade (r:1 w:1)
	// Storage: AwesomeAvatars Avatars (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:0)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	// Storage: AwesomeAvatars Owners (r:2 w:2)
	// Storage: AwesomeAvatars Accounts (r:2 w:2)
	/// The range of component `n` is `[1, 100]`.
	fn buy(n: u32, ) -> Weight {
		// Minimum execution time: 207_877 nanoseconds.
		Weight::from_ref_time(454_309_724 as u64)
			// Standard Error: 151_022
			.saturating_add(Weight::from_ref_time(2_517_327 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(13 as u64))
			.saturating_add(RocksDbWeight::get().writes(10 as u64))
	}
	// Storage: AwesomeAvatars Accounts (r:1 w:1)
	// Storage: AwesomeAvatars GlobalConfigs (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	fn upgrade_storage() -> Weight {
		// Minimum execution time: 194_128 nanoseconds.
		Weight::from_ref_time(218_627_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: AwesomeAvatars Organizer (r:0 w:1)
	fn set_organizer() -> Weight {
		// Minimum execution time: 50_164 nanoseconds.
		Weight::from_ref_time(56_488_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars Treasurer (r:0 w:1)
	fn set_treasurer() -> Weight {
		// Minimum execution time: 55_186 nanoseconds.
		Weight::from_ref_time(61_604_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars Treasurer (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonId (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:0)
	// Storage: AwesomeAvatars CurrentSeasonStatus (r:1 w:0)
	// Storage: AwesomeAvatars Treasury (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn claim_treasury() -> Weight {
		// Minimum execution time: 166_460 nanoseconds.
		Weight::from_ref_time(191_121_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: AwesomeAvatars Organizer (r:1 w:0)
	// Storage: AwesomeAvatars Seasons (r:1 w:1)
	fn set_season() -> Weight {
		// Minimum execution time: 82_258 nanoseconds.
		Weight::from_ref_time(92_048_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars Organizer (r:1 w:0)
	// Storage: AwesomeAvatars GlobalConfigs (r:0 w:1)
	fn update_global_config() -> Weight {
		// Minimum execution time: 69_141 nanoseconds.
		Weight::from_ref_time(75_555_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: AwesomeAvatars Organizer (r:1 w:0)
	// Storage: AwesomeAvatars Accounts (r:1 w:1)
	fn set_free_mints() -> Weight {
		// Minimum execution time: 69_824 nanoseconds.
		Weight::from_ref_time(76_287_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
