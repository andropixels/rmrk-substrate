use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

use super::*;
use mock::{Event as MockEvent, *};

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

fn next_block() {
	System::set_block_number(System::block_number() + 1);
}

fn fast_forward_to(n: u64) {
	while System::block_number() < n {
		next_block();
	}
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
		assert_ok!(PhalaWorld::flip_claim_spirits_status(Origin::root()));
		// Dispatch a claim spirit
		assert_ok!(PhalaWorld::claim_spirit(Origin::signed(ALICE), 1, bvec![0u8; 20], bvec![0u8; 20]));
	});
}

#[test]
fn claimed_spirit_twice_fails() {
	ExtBuilder::default().build().execute_with(|| {
		// Enable spirits to be claimed
		assert_ok!(PhalaWorld::flip_claim_spirits_status(Origin::root()));
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
