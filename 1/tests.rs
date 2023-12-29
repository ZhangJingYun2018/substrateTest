
use super::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_work() {
	new_test_ext().execute_with(|| {
		let cliam = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), cliam.clone()));

		assert_eq!(
			Proofs::<Test>::get(&cliam),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn create_claim_already_work() {
	new_test_ext().execute_with(|| {
		let cliam = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), cliam.clone());

		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), cliam.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn revoke_claim_exsit_work() {
	new_test_ext().execute_with(|| {
		let cliam = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), cliam.clone());
		let _ = PoeModule::revoke_claim(RuntimeOrigin::signed(1), cliam.clone());
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), cliam.clone()),
			Error::<Test>::ClaimNotExsit
		);
	})
}

#[test]
fn revoke_claim_owner_work() {
	new_test_ext().execute_with(|| {
		let cliam = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), cliam.clone());
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(2), cliam.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn devolve_claim_work() {
	new_test_ext().execute_with(|| {
		let cliam = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), cliam.clone());
		let _ = PoeModule::revoke_claim(RuntimeOrigin::signed(1), cliam.clone());
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(2), cliam.clone());
		assert_eq!(
			Proofs::<Test>::get(&cliam),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), cliam.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}
