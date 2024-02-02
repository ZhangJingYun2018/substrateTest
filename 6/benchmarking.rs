//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_claim() {
		let cliam = BoundedVec::try_from(vec![0, 1]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		create_claim(RawOrigin::Signed(caller), cliam);

	}

	#[benchmark]
	fn revoke_claim() {
		let cliam = BoundedVec::try_from(vec![0, 1]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		Template::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(),cliam.clone());
		#[extrinsic_call]
		revoke_claim(RawOrigin::Signed(caller), cliam);

	}

	#[benchmark]
	fn transfer_claim() {
		let cliam = BoundedVec::try_from(vec![0, 1]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		Template::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(),cliam.clone());
		let target: T::AccountId = account("target", 0, 0);
		#[extrinsic_call]
		transfer_claim(RawOrigin::Signed(caller), cliam,target);


	}


	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
