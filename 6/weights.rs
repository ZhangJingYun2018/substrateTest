
//! Autogenerated weights for `pallet_poe`
//!
//! 
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-30, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `zjy-HPZHAN99Mobile`, CPU: `AMD Ryzen 7 5800H with Radeon Graphics`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_poe
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// pallets/poe/src/weights.rs
// --template
// .maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_poe`.
pub trait WeightInfo {
	fn create_claim() -> Weight;
	fn revoke_claim() -> Weight;
	fn transfer_claim() -> Weight;
}

/// Weights for `pallet_poe` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `TemplatePoe::Proofs` (r:1 w:1)
	/// Proof: `TemplatePoe::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	fn create_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3528`
		// Minimum execution time: 16_260_000 picoseconds.
		Weight::from_parts(19_086_000, 3528)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `TemplatePoe::Proofs` (r:1 w:1)
	/// Proof: `TemplatePoe::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	fn revoke_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `3528`
		// Minimum execution time: 16_651_000 picoseconds.
		Weight::from_parts(18_855_000, 3528)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `TemplatePoe::Proofs` (r:1 w:1)
	/// Proof: `TemplatePoe::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	fn transfer_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `3528`
		// Minimum execution time: 11_843_000 picoseconds.
		Weight::from_parts(12_213_000, 3528)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `TemplatePoe::Proofs` (r:1 w:1)
	/// Proof: `TemplatePoe::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	fn create_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3528`
		// Minimum execution time: 16_260_000 picoseconds.
		Weight::from_parts(19_086_000, 3528)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `TemplatePoe::Proofs` (r:1 w:1)
	/// Proof: `TemplatePoe::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	fn revoke_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `3528`
		// Minimum execution time: 16_651_000 picoseconds.
		Weight::from_parts(18_855_000, 3528)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `TemplatePoe::Proofs` (r:1 w:1)
	/// Proof: `TemplatePoe::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	fn transfer_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `3528`
		// Minimum execution time: 11_843_000 picoseconds.
		Weight::from_parts(12_213_000, 3528)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
