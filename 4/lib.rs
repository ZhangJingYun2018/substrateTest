#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
use frame_system::offchain::{
	AppCrypto, CreateSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer,
	SigningTypes,
};

use sp_runtime::offchain::{http, Duration};
pub use weights::*;

use serde::{Deserialize, Deserializer};

use sp_core::crypto::KeyTypeId;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"pric");
pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::Vec, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct Payload<Public> {
		number: u64,
		public: Public,
	}

	impl<T: SigningTypes> SignedPayload<T> for Payload<T::Public> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}
	#[derive(Deserialize, Encode, Decode)]
	struct PriceInfo {
		pub price: u64,
	}

	use core::{convert::TryInto, fmt};
	impl fmt::Debug for PriceInfo {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(f, "{{ price is : {} }}", &self.price)
		}
	}

	#[derive(Deserialize, Encode, Decode)]
	struct GithubInfo {
		#[serde(deserialize_with = "de_string_to_bytes")]
		login: Vec<u8>,
		#[serde(deserialize_with = "de_string_to_bytes")]
		blog: Vec<u8>,
		public_repos: u32,
	}

	pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: &str = Deserialize::deserialize(de)?;
		Ok(s.as_bytes().to_vec())
	}

	impl fmt::Debug for GithubInfo {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(
				f,
				"{{ login: {}, blog: {}, public_repos: {} }}",
				sp_std::str::from_utf8(&self.login).map_err(|_| fmt::Error)?,
				sp_std::str::from_utf8(&self.blog).map_err(|_| fmt::Error)?,
				&self.public_repos
			)
		}
	}
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn ocwvalue)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type OCWValue<T> = StorageValue<_, u64>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;
			sp_io::offchain_index::set(b"12345678",&something.encode());
			log::info!(
				"写入的数据是: {:?}",
				something.encode()
			);
			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn unsigned_extrinsic_with_signed_payload(
			origin: OriginFor<T>,
			payload: Payload<T::Public>,
			_signature: T::Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			log::info!(
				"OCW ==> in call unsigned_extrinsic_with_signed_payload: {:?}",
				payload.number
			);
			OCWValue::<T>::put(payload.number);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		// #[pallet::call_index(2)]
		// #[pallet::weight(T::WeightInfo::do_something())]
		// pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
		// 	// Check that the extrinsic was signed and get the signer.
		// 	// This function will return an error if the extrinsic is not signed.
		// 	// https://docs.substrate.io/main-docs/build/origins/
		// 	let who = ensure_signed(origin)?;
		// 	// even
		// 	let key = Self::derive_key(block_number - 1u32.into());
		// 	let mut val_ref = StorageValueRef::persistent(&key);

		// 	// get from db by key
		// 	if let Ok(Some(value)) = val_ref.get::<([u8;32], u64)>() {
		// 		// print values
		// 		log::info!("OCW ==> in even block, value read: {:?}", value);
		// 		// delete that key
		// 		val_ref.clear();
		// 	}
		// 	// Update storage.
		// 	<Something<T>>::put(something);

		// 	// Emit an event.
		// 	Self::deposit_event(Event::SomethingStored { something, who });
		// 	// Return a successful DispatchResultWithPostInfo
		// 	Ok(())
		// }
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		/// Validate unsigned call to this module.
		///
		/// By default unsigned transactions are disallowed, but implementing the validator
		/// here we make sure that some particular calls (the ones produced by offchain worker)
		/// are being whitelisted and marked as valid.
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			const UNSIGNED_TXS_PRIORITY: u64 = 100;
			let valid_tx = |provide| {
				ValidTransaction::with_tag_prefix("my-pallet")
					.priority(UNSIGNED_TXS_PRIORITY) // please define `UNSIGNED_TXS_PRIORITY` before this line
					.and_provides([&provide])
					.longevity(3)
					.propagate(true)
					.build()
			};

			// match call {
			// 	Call::submit_data_unsigned { key: _ } => valid_tx(b"my_unsigned_tx".to_vec()),
			// 	_ => InvalidTransaction::Call.into(),
			// }

			match call {
				Call::unsigned_extrinsic_with_signed_payload { ref payload, ref signature } => {
					if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
						return InvalidTransaction::BadProof.into();
					}
					valid_tx(b"unsigned_extrinsic_with_signed_payload".to_vec())
				},
				_ => InvalidTransaction::Call.into(),
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: BlockNumberFor<T>) {
			// if  block_number % 1u32.into()  == sp_runtime::traits::One::one() {
				
			log::info!("OCW ==> Hello World from offchain workers!: {:?}", block_number);
			//github 信息
			if let Ok(info) = Self::fetch_github_info() {
				log::info!("OCW ==> Github Info: {:?}", info);
			} else {
				log::info!("OCW ==> Error while fetch github info!");
			}
			//价格信息
			if let Ok(info) = Self::get_price_info() {
				log::info!("OCW ==> Github Info: {:?}", info);
			} else {
				log::info!("OCW ==> Error while fetch price info!");
			}
			// odd
			let key = Self::derive_key(block_number);
			let val_ref = sp_runtime::offchain::storage::StorageValueRef::persistent(&key);

			//  get a local random value
			let random_slice = sp_io::offchain::random_seed();

			//  get a local timestamp
			let timestamp_u64 = sp_io::offchain::timestamp().unix_millis();

			// combine to a tuple and print it
			let value = (random_slice, timestamp_u64);
			log::info!("OCW ==> in odd block, value to write: {:?}", value);

			struct StateError;

			//  write or mutate tuple content to key
			let res = val_ref.mutate(
				|val: Result<
					Option<([u8; 32], u64)>,
					sp_runtime::offchain::storage::StorageRetrievalError,
				>|
				 -> Result<_, StateError> {
					match val {
						Ok(Some(_)) => Ok(value),
						_ => Ok(value),
					}
				},
			);

			match res {
				Ok(value) => {
					log::info!("OCW ==> in odd block, mutate successfully: {:?}", value);
				},
				Err(sp_runtime::offchain::storage::MutateStorageError::ValueFunctionFailed(_)) =>
					(),
				Err(sp_runtime::offchain::storage::MutateStorageError::ConcurrentModification(
					_,
				)) => (),
			}

			let number: u64 = 32;
			// Retrieve the signer to sign the payload
			let signer = Signer::<T, T::AuthorityId>::any_account();

			if let Some((_, res)) = signer.send_unsigned_transaction(
				// this line is to prepare and return payload
				|acct| Payload { number, public: acct.public.clone() },
				|payload, signature| Call::unsigned_extrinsic_with_signed_payload {
					payload,
					signature,
				},
			) {
				match res {
					Ok(()) => {
						log::info!("OCW ==> unsigned tx with signed payload successfully sent.");
					},
					Err(()) => {
						log::error!("OCW ==> sending unsigned tx with signed payload failed.");
					},
				};
			} else {
				// The case of `None`: no account is available for sending
				log::error!("OCW ==> No local account available");
			}
			sp_io::offchain_index::set(b"12345678",&[1;8]);
			let val_ref1 = sp_runtime::offchain::storage::StorageValueRef::persistent(b"12345678").get::<u8>();
			log::info!("OCW ==> 链上写入的数据 {:?}", val_ref1);
			log::info!("OCW ==> 链上写入的数据 {:?}", OCWValue::<T>::get());
			log::info!("OCW ==> Leave from offchain workers!: {:?}", block_number);
		// }
		}
	}

	impl<T: Config> Pallet<T> {
		#[deny(clippy::clone_double_ref)]
		fn derive_key(block_number: BlockNumberFor<T>) -> Vec<u8> {
			block_number.using_encoded(|encoded_bn| {
				b"node-template::storage::"
					.iter()
					.chain(encoded_bn)
					.copied()
					.collect::<Vec<u8>>()
			})
		}

		fn fetch_github_info() -> Result<GithubInfo, http::Error> {
			// prepare for send request
			let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(8_000));
			let request = http::Request::get("https://api.github.com/orgs/substrate-developer-hub");
			let pending = request
				.add_header("User-Agent", "Substrate-Offchain-Worker")
				.deadline(deadline)
				.send()
				.map_err(|_| http::Error::IoError)?;
			let response =
				pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
			if response.code != 200 {
				log::warn!("Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown)
			}
			let body = response.body().collect::<Vec<u8>>();
			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				http::Error::Unknown
			})?;

			// parse the response str
			let gh_info: GithubInfo =
				serde_json::from_str(body_str).map_err(|_| http::Error::Unknown)?;

			Ok(gh_info)
		}

		fn get_price_info() -> Result<PriceInfo, http::Error> {
			// prepare for send request
			let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(8_000));
			let request = http::Request::get("https://api.github.com/orgs/substrate-developer-hub");
			let pending = request
				.add_header("User-Agent", "Substrate-Offchain-Worker")
				.deadline(deadline)
				.send()
				.map_err(|_| http::Error::IoError)?;
			let response =
				pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
			if response.code != 200 {
				log::warn!("Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown)
			}
			let body = response.body().collect::<Vec<u8>>();
			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				http::Error::Unknown
			})?;

			// parse the response str
			let price_info: PriceInfo =
				serde_json::from_str(body_str).map_err(|_| http::Error::Unknown)?;

			Ok(price_info)
		}
	}
}
