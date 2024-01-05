
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_kitty() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 0;
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		System::assert_last_event(
			Event::KittyCreated {
				who: account_id,
				kitty_id,
				kitty: KittiesModule::kitties(kitty_id).unwrap(),
			}
			.into(),
		);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create_kitty(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn transfer_kitty() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_1 = 1;
		let account_2 = 2;
	
		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_1), account_2, kitty_id));
		System::assert_last_event(
			Event::KittyTransfered { who: account_1, recipient: account_2, kitty_id }.into(),
		);
		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(account_2), account_1, kitty_id),
			Error::<Test>::InvalidKittyId
		);
	
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_2));
	})
}
