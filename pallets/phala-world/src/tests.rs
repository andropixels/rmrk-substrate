use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, assert_storage_noop};
use sp_core::crypto::AccountId32;
use sp_core::Pair;

use super::*;
use mock::{
	Event as MockEvent, ExtBuilder, Balances, BalanceCall, Call, PhalaWorld, Origin, SystemCall, Test,
};

/// Turns a string into a BoundedVec
fn stb(s: &str) -> BoundedVec<u8, ValueLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a BoundedVec
fn stbk(s: &str) -> BoundedVec<u8, KeyLimit> {
	s.as_bytes().to_vec().try_into().unwrap()
}

/// Turns a string into a Vec
fn stv(s: &str) -> Vec<u8> {
	s.as_bytes().to_vec()
}

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

fn mint_collection(account: AccountId32) {
	// Mint Spirits collection
	RmrkCore::create_collection(Origin::signed(account), bvec![0u8; 20], Some(5), bvec![0u8; 15]);
}

#[test]
fn claimed_spirit_works() {
	ExtBuilder::default().build(ALICE).execute_with(|| {
		let overlord_pair = sr25519::Pair::from_seed(b"12345678901234567890123456789012");
		// let overlord_pub = overlord_pair.public();
		// Enable spirits to be claimed
		assert_ok!(PhalaWorld::set_status_type(Origin::signed(ALICE), true, StatusType::ClaimSpirits));
		//assert_ok!(PhalaWorld::set_overlord(Origin::signed(ALICE), AccountId32::new(overlord_pub)));

		let metadata = stb("I am Overlord");
		let claim = Encode::encode(&(ALICE, metadata.clone()));
		let overlord_signature = overlord_pair.sign(&claim);

		// Mint collection with Overlord account ALICE
		mint_collection(ALICE);
		// Dispatch a claim spirit
		assert_ok!(PhalaWorld::claim_spirit(Origin::signed(ALICE), 1, overlord_signature, bvec![0u8; 20]));
	});
}

#[test]
fn claimed_spirit_twice_fails() {
	ExtBuilder::default().build(ALICE).execute_with(|| {
		let overlord_pair = sr25519::Pair::from_seed(b"12345678901234567890123456789012");
		// let overlord_pub = overlord_pair.public();
		let metadata = stb("I am Overlord");
		let claim = Encode::encode(&(BOB, metadata.clone()));
		let overlord_signature = overlord_pair.sign(&claim);
		// Set the Overlord Admin account
		assert_ok!(PhalaWorld::set_overlord(Origin::signed(ALICE), BOB));
		// Alice can no longer set the Overlord Admin account
		assert_noop!(PhalaWorld::set_overlord(Origin::signed(ALICE), BOB), Error::<Test>::RequireOverlordAccount);
		// Last Event Overlord changed from None
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::OverlordChanged {
			old_overlord: Some(ALICE),
		}));
		// Mint Spirits collection
		mint_collection(BOB);
		// Enable spirits to be claimed
		assert_ok!(PhalaWorld::set_status_type(Origin::signed(BOB), true, StatusType::ClaimSpirits));
		// Dispatch a claim spirit
		assert_ok!(PhalaWorld::claim_spirit(Origin::signed(ALICE), 1, overlord_signature.clone(), bvec![0u8; 20]));
		// Fail to dispatch a second claim spirit
		assert_noop!(PhalaWorld::claim_spirit(Origin::signed(ALICE), 1, overlord_signature, bvec![0u8; 20]), Error::<Test>::SpiritAlreadyClaimed);
	});
}

#[test]
fn start_world_clock_works() {
	ExtBuilder::default().build(ALICE).execute_with(|| {
		// Initialize the Phala World Clock
		assert_ok!(PhalaWorld::initialize_world_clock(Origin::root()));
	});
}

#[test]
fn auto_increment_era_works() {
	ExtBuilder::default().build(ALICE).execute_with(|| {
		// Set Overlord admin as ALICE
		assert_ok!(PhalaWorld::set_overlord(Origin::root(), BOB));
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::OverlordChanged {
			old_overlord: Some(ALICE),
		}));
		// Initialize the Phala World Clock
		assert_ok!(PhalaWorld::initialize_world_clock(Origin::signed(BOB)));
		// Check Zero Day is Some(1)
		assert_eq!(PhalaWorld::zero_day(), Some(1));
		// Go to block 7 that would increment the Era at Block 6
		fast_forward_to(7);
		// Check Era is 1
		assert_eq!(PhalaWorld::era(), 1);
		// Check if event triggered
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::NewEra {
			time: 6,
			era: 1,
		}));
	});
}