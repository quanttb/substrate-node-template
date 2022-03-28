use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_kitty_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(KittiesModule::create_kitty(Origin::signed(1)));

		let owned_kitties = KittiesModule::kitties_owned(1);
		assert_eq!(owned_kitties.len(), 1);

		let id = owned_kitties[0];
		let kitty = KittiesModule::kitties(id).unwrap();

		assert_eq!(kitty.owner, 1);
	});
}

#[test]
fn set_price_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let kitty_id = KittiesModule::mint(&1, None, None).unwrap();

		assert_ok!(KittiesModule::set_price(Origin::signed(1), kitty_id, Some(10)));

		let kitty = KittiesModule::kitties(kitty_id).unwrap();

		assert_eq!(kitty.price, Some(10));
	});
}

#[test]
fn transfer_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let kitty_id = KittiesModule::mint(&1, None, None).unwrap();

		assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, kitty_id));

		let kitty = KittiesModule::kitties(kitty_id).unwrap();

		assert_eq!(kitty.owner, 2);
	});
}

#[test]
fn buy_kitty_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// let _ = 1.borrow();

		let kitty_id = KittiesModule::mint(&1, None, None).unwrap();

		assert_ok!(KittiesModule::set_price(Origin::signed(1), kitty_id, Some(10)));

		let old_balance = Balances::free_balance(&2);

		assert_ok!(KittiesModule::buy_kitty(Origin::signed(2), kitty_id, 10));

		let new_balance = Balances::free_balance(&2);

		let kitty = KittiesModule::kitties(kitty_id).unwrap();

		assert_eq!(kitty.owner, 2);
		assert_eq!(old_balance - 10, new_balance);
	});
}

#[test]
fn breed_kitty_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let first_kitty_id = KittiesModule::mint(&1, None, None).unwrap();

		System::set_block_number(2);

		let second_kitty_id = KittiesModule::mint(&1, None, None).unwrap();

		assert_ok!(KittiesModule::breed_kitty(Origin::signed(1), first_kitty_id, second_kitty_id));

		let owned_kitties = KittiesModule::kitties_owned(1);
		assert_eq!(owned_kitties.len(), 3);

		let new_kitty_id = owned_kitties[2];
		let new_kitty = KittiesModule::kitties(new_kitty_id).unwrap();

		assert_eq!(new_kitty.owner, 1);
	});
}
