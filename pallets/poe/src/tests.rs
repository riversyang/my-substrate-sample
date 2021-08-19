use crate::{mock::*, Error, Proofs};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Pallet::<Test>::block_number()));
	});
}

#[test]
fn create_claim_which_already_exists() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert!(!Proofs::<Test>::contains_key(&claim),);
	});
}

#[test]
fn revoke_claim_which_not_exists() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::NoSuchProof
		);
	});
}

#[test]
fn claim_max_length_works() {
	let claim = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
	new_test_ext().execute_with(|| {
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofTooLong
		);
	});
}
