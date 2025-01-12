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

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod traits;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use crate::{traits::*, weights::WeightInfo};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			tokens::{
				nonfungibles_v2::{Create, Destroy, Inspect, Mutate},
				AttributeNamespace,
			},
			Locker,
		},
		PalletId,
	};
	use frame_system::{ensure_root, ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::{
		traits::{AccountIdConversion, AtLeast32BitUnsigned},
		Saturating,
	};

	pub type EncodedAssetOf<T> = BoundedVec<u8, <T as Config>::MaxAssetEncodedSize>;

	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Copy, Clone, Debug, Eq, PartialEq)]
	pub enum PalletLockedState {
		/// Pallet is unlocked, all operations can be performed
		Unlocked,
		/// Pallet is locked, operations are restricted
		Locked,
	}

	impl Default for PalletLockedState {
		fn default() -> Self {
			PalletLockedState::Unlocked
		}
	}

	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Debug, Eq, PartialEq)]
	pub enum NftStatus {
		/// The NFT exists in storage in the chain
		Stored,
		/// The NFT has been uploaded outside the chain
		Uploaded,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum amount of bytes that an asset may be encoded as.
		#[pallet::constant]
		type MaxAssetEncodedSize: Get<u32>;

		/// Identifier for the collection of an NFT.
		type CollectionId: Member + Parameter + Copy + MaxEncodedLen + AtLeast32BitUnsigned;

		/// Type that holds the specific configurations for a collection.
		type CollectionConfig: Copy
			+ Clone
			+ Default
			+ PartialEq
			+ Encode
			+ Decode
			+ MaxEncodedLen
			+ TypeInfo;

		/// Identifier for the individual instances of an NFT.
		type ItemId: Member + Parameter + Default + Copy + MaxEncodedLen + AtLeast32BitUnsigned;

		/// Type that holds the specific configurations for an item.
		type ItemConfig: Copy
			+ Clone
			+ Default
			+ PartialEq
			+ Encode
			+ Decode
			+ MaxEncodedLen
			+ TypeInfo;

		type NftHelper: Inspect<Self::AccountId, CollectionId = Self::CollectionId, ItemId = Self::ItemId>
			+ Create<Self::AccountId, Self::CollectionConfig>
			+ Mutate<Self::AccountId, Self::ItemConfig>
			+ Destroy<Self::AccountId>;

		/// The holding's pallet id, used for deriving its sovereign account identifier for the Nft
		/// holding account.
		#[pallet::constant]
		type HoldingPalletId: Get<PalletId>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn organizer)]
	pub type Organizer<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn lock_status)]
	pub type LockedState<T: Config> = StorageValue<_, PalletLockedState, ValueQuery>;

	#[pallet::storage]
	pub type NextItemId<T: Config> =
		StorageMap<_, Identity, T::CollectionId, T::ItemId, ValueQuery>;

	#[pallet::storage]
	pub type LockItemStatus<T: Config> =
		StorageDoubleMap<_, Identity, T::CollectionId, Identity, T::ItemId, NftStatus, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn holding_account)]
	pub type HoldingAccount<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn nft_claimants)]
	pub type NftClaimants<T: Config> = StorageDoubleMap<
		_,
		Identity,
		T::CollectionId,
		Identity,
		T::ItemId,
		T::AccountId,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An organizer has been set.
		OrganizerSet { organizer: T::AccountId },
		/// The pallet's lock status has been set
		LockedStateSet { locked_state: PalletLockedState },
		/// Asset has been stored as an NFT [collection_id, asset_id, owner]
		AssetStored { collection_id: T::CollectionId, asset_id: T::ItemId, owner: T::AccountId },
		/// Asset has been restored back from its NFT representation [collection_id, asset_id,
		/// owner]
		AssetRestored { collection_id: T::CollectionId, asset_id: T::ItemId, owner: T::AccountId },
		/// Asset has been transferred outside the chain [collection_id, asset_id, owner]
		AssetTransferred {
			collection_id: T::CollectionId,
			asset_id: T::ItemId,
			owner: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// There is no account set as the organizer
		OrganizerNotSet,
		/// The pallet is currently locked and cannot be interacted with.
		PalletLocked,
		/// The given asset resulted in an encoded size larger that the defined encoding limit.
		AssetSizeAboveEncodingLimit,
		/// The given NFT id didn't match any entries for the specified collection.
		NftNotFound,
		/// The given NFT id doesn't have the proper attribute set.
		NftAttributeMissing,
		/// The given NFT is not owned by the requester.
		NftNotOwned,
		/// The given NFT is currently outside of the chain, transfer it back before attempting a
		/// restore.
		NftOutsideOfChain,
		/// The process of restoring an NFT into an Asset has failed.
		AssetRestoreFailure,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::set_organizer())]
		pub fn set_organizer(origin: OriginFor<T>, organizer: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			Organizer::<T>::put(&organizer);
			Self::deposit_event(Event::OrganizerSet { organizer });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::set_locked_state())]
		pub fn set_locked_state(
			origin: OriginFor<T>,
			locked_state: PalletLockedState,
		) -> DispatchResult {
			Self::ensure_organizer(origin)?;
			LockedState::<T>::put(locked_state);
			Self::deposit_event(Event::LockedStateSet { locked_state });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// The account identifier of the holding account.
		pub fn holding_account_id() -> T::AccountId {
			if let Some(account) = Self::holding_account() {
				account
			} else {
				let account: T::AccountId = T::HoldingPalletId::get().into_account_truncating();

				HoldingAccount::<T>::put(account.clone());

				account
			}
		}

		fn ensure_organizer(origin: OriginFor<T>) -> DispatchResult {
			let maybe_organizer = ensure_signed(origin)?;
			let existing_organizer = Self::organizer().ok_or(Error::<T>::OrganizerNotSet)?;
			ensure!(maybe_organizer == existing_organizer, DispatchError::BadOrigin);
			Ok(())
		}

		fn ensure_unlocked() -> DispatchResult {
			ensure!(
				LockedState::<T>::get() == PalletLockedState::Unlocked,
				Error::<T>::PalletLocked
			);
			Ok(())
		}
	}

	impl<T: Config, Asset: NftConvertible> NftHandler<T::AccountId, Asset> for Pallet<T> {
		type CollectionId = T::CollectionId;
		type AssetId = T::ItemId;
		type AssetConfig = T::ItemConfig;

		fn store_as_nft(
			owner: T::AccountId,
			collection_id: Self::CollectionId,
			asset: Asset,
		) -> Result<Self::AssetId, DispatchError> {
			Pallet::<T>::ensure_unlocked()?;

			let encoded_attributes = asset.get_encoded_attributes();

			let encoded_asset: EncodedAssetOf<T> = asset
				.encode_into()
				.try_into()
				.map_err(|_| Error::<T>::AssetSizeAboveEncodingLimit)?;

			let next_item_id = NextItemId::<T>::mutate(collection_id, |item_id| {
				let next_item_id = *item_id;
				item_id.saturating_inc();
				next_item_id
			});

			T::NftHelper::mint_into(
				&collection_id,
				&next_item_id,
				&Self::holding_account_id(),
				&Self::AssetConfig::default(),
				true,
			)?;

			T::NftHelper::set_typed_attribute(
				&collection_id,
				&next_item_id,
				&Asset::ASSET_CODE,
				&encoded_asset,
			)?;

			for (attribute_key, attribute) in encoded_attributes {
				T::NftHelper::set_typed_attribute(
					&collection_id,
					&next_item_id,
					&attribute_key,
					&attribute,
				)?;
			}

			LockItemStatus::<T>::insert(collection_id, next_item_id, NftStatus::Stored);
			NftClaimants::<T>::insert(collection_id, next_item_id, owner.clone());

			Self::deposit_event(Event::<T>::AssetStored {
				collection_id,
				asset_id: next_item_id,
				owner,
			});

			Ok(next_item_id)
		}

		fn recover_from_nft(
			owner: T::AccountId,
			collection_id: Self::CollectionId,
			asset_id: Self::AssetId,
		) -> Result<Asset, DispatchError> {
			Pallet::<T>::ensure_unlocked()?;

			ensure!(
				NftClaimants::<T>::get(collection_id, asset_id) == Some(owner.clone()),
				Error::<T>::NftNotOwned
			);
			ensure!(
				LockItemStatus::<T>::get(collection_id, asset_id) == Some(NftStatus::Stored),
				Error::<T>::NftOutsideOfChain
			);

			let encoded_asset_data = T::NftHelper::typed_attribute::<AssetCode, EncodedAssetOf<T>>(
				&collection_id,
				&asset_id,
				&AttributeNamespace::Pallet,
				&Asset::ASSET_CODE,
			)
			.ok_or(Error::<T>::NftAttributeMissing)?;

			let asset = Asset::decode_from(encoded_asset_data.into_inner())
				.map_err(|_| Error::<T>::AssetRestoreFailure)?;

			T::NftHelper::clear_typed_attribute(&collection_id, &asset_id, &Asset::ASSET_CODE)?;

			for attribute_key in Asset::get_attribute_table() {
				T::NftHelper::clear_typed_attribute(&collection_id, &asset_id, &attribute_key)?;
			}

			T::NftHelper::burn(&collection_id, &asset_id, Some(&Self::holding_account_id()))?;
			LockItemStatus::<T>::remove(collection_id, asset_id);

			Self::deposit_event(Event::<T>::AssetRestored { collection_id, asset_id, owner });

			Ok(asset)
		}

		fn schedule_nft_upload(
			_owner: T::AccountId,
			_collection_id: Self::CollectionId,
			_asset_id: Self::AssetId,
		) -> DispatchResult {
			Pallet::<T>::ensure_unlocked()?;
			todo!()
		}
	}

	impl<T: Config> Locker<T::CollectionId, T::ItemId> for Pallet<T> {
		fn is_locked(collection: T::CollectionId, item: T::ItemId) -> bool {
			LockItemStatus::<T>::get(collection, item).is_some()
		}
	}
}
