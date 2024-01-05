#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::{OptionQuery, *};
	use frame_system::pallet_prelude::*;

	use frame_support::traits::Randomness;
	use sp_io::hashing::blake2_128;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	pub type KittyId = u32;

	#[derive(
		Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen,
	)]
	pub struct Kitty(pub [u8; 16]);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
	}

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T: Config> = StorageValue<_, KittyId, ValueQuery>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KilltOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_parents)]
	pub type KilltParents<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId), OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyBred { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyTransfered { who: T::AccountId, recipient: T::AccountId, kitty_id: KittyId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		SameKityId,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(1000+T::DbWeight::get().writes(1).ref_time())]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_id = Self::get_next_id()?;
			let kitty = Kitty(Self::random_value(&who));
			Kitties::<T>::insert(kitty_id, kitty);
			KilltOwner::<T>::insert(kitty_id, &who);
			Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(1000+T::DbWeight::get().writes(1).ref_time())]
		pub fn bred(
			origin: OriginFor<T>,
			kitty_id_1: KittyId,
			kitty_id_2: KittyId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKityId);
			ensure!(Kitties::<T>::contains_key(kitty_id_1), Error::<T>::InvalidKittyId);
			ensure!(Kitties::<T>::contains_key(kitty_id_2), Error::<T>::InvalidKittyId);

			let kitty_id = Self::get_next_id()?;
			let kitty_1 = Self::kitties(kitty_id_1).unwrap();
			let kitty_2 = Self::kitties(kitty_id_2).unwrap();

			let selector = Self::random_value(&who);
			let mut data = [0u8; 16];
			for i in 0..kitty_1.0.len() {
				data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & selector[i]);
			}
			let kitty = Kitty(data);
			Kitties::<T>::insert(kitty_id, kitty);
			KilltOwner::<T>::insert(kitty_id, &who);
			KilltParents::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));
			Self::deposit_event(Event::KittyBred { who, kitty_id, kitty });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(1000+T::DbWeight::get().writes(1).ref_time())]
		pub fn transfer(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			kitty_id: KittyId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);

			if let Some(owner) = Self::kitty_owner(kitty_id) {
				ensure!(!(owner == who), Error::<T>::InvalidKittyId);
			};

			KilltOwner::<T>::insert(kitty_id, &recipient);

			Self::deposit_event(Event::KittyTransfered { who, recipient, kitty_id });
			Ok(())
		}
	}
	impl<T: Config> Pallet<T> {
		fn get_next_id() -> Result<KittyId, DispatchError> {
			NextKittyId::<T>::try_mutate(|next_id| -> Result<KittyId, DispatchError> {
				let current_id = *next_id;
				*next_id = next_id
					.checked_add(1)
					.ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
				Ok(current_id)
			})
		}

		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}
	}
}
