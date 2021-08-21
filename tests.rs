use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;


#[test]
fn limit_vec_size_works() {
	new_test_ext().execute_with(|| {

		// vec size is limited to less or equal to 10
		let claim = vec![0,1,2,3,4,5,6,7,8,9,10];

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimOverSize
		);
	})
}

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];

		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim), 
			(1,frame_system::Pallet::<Test>::block_number())
		);
	})
}



#[test]
fn create_claim_failed_when_claim_already_exist(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);
	})
}


#[test]
fn revoke_claim_works(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim), 
			(0u64,0u64)
		);

	})			
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::NoSuchProof
		);

	})
}

#[test]
fn revoke_claim_failed_when_sender_not_owner(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotProofOwner
		);		

	})
}


#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let another_acount = 1234;
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(),another_acount));
	})
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let another_acount = 1234;
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(),another_acount),
			Error::<Test>::NoSuchProof
		);

	})
}

#[test]
fn transfer_claim_failed_when_sender_not_owner(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let another_acount = 1234;
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(),another_acount),
			Error::<Test>::NotProofOwner
		);

	})
}
