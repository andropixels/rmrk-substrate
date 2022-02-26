use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, assert_storage_noop};

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

#[test]
fn claimed_spirit_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Enable spirits to be claimed
		assert_ok!(PhalaWorld::set_claim_spirits_status(Origin::root(), true));
		// Dispatch a claim spirit
		assert_ok!(PhalaWorld::claim_spirit(Origin::signed(ALICE), 1, bvec![0u8; 20], bvec![0u8; 20]));
	});
}

#[test]
fn claimed_spirit_twice_fails() {
	ExtBuilder::default().build().execute_with(|| {
		// Enable spirits to be claimed
		assert_ok!(PhalaWorld::set_claim_spirits_status(Origin::root(), true));
		// Dispatch a claim spirit
		assert_ok!(PhalaWorld::claim_spirit(Origin::signed(ALICE), 1, bvec![0u8; 20], bvec![0u8; 20]));
		// Fail to dispatch a second claim spirit
		assert_noop!(PhalaWorld::claim_spirit(Origin::signed(ALICE), 1, bvec![0u8; 20], bvec![0u8; 20]), Error::<Test>::SpiritAlreadyClaimed);
	});
}

#[test]
fn start_world_clock_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Initialize the Phala World Clock
		assert_ok!(PhalaWorld::initialize_world_clock(Origin::root()));
	});
}

#[test]
fn auto_increment_era_works() {
	ExtBuilder::default().build().execute_with(|| {
		// Initialize the Phala World Clock
		assert_ok!(PhalaWorld::initialize_world_clock(Origin::root()));
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