use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use sp_core::{crypto::AccountId32, Pair};

use super::*;
use mock::{Balances, Call, Event as MockEvent, ExtBuilder, Origin, PhalaWorld, Test};

/// Turns a string into a BoundedVec
fn stb(s: &str) -> BoundedVec<u8, UniquesStringLimit> {
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

fn metadata_accounts(
	mut alice_metadata: BoundedVec<u8, UniquesStringLimit>,
	mut bob_metadata: BoundedVec<u8, UniquesStringLimit>,
	mut charlie_metadata: BoundedVec<u8, UniquesStringLimit>,
) {
	alice_metadata = stb("I am ALICE");
	bob_metadata = stb("I am BOB");
	charlie_metadata = stb("I am CHARLIE");
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
		assert_ok!(PhalaWorld::set_status_type(
			Origin::signed(ALICE),
			true,
			StatusType::ClaimSpirits
		));

		let metadata = stb("I am Overlord");
		let claim = Encode::encode(&(BOB, metadata.clone()));
		let overlord_signature = overlord_pair.sign(&claim);

		// Mint collection with Overlord account ALICE
		mint_collection(ALICE);
		// Dispatch a claim spirit from BOB's account
		assert_ok!(PhalaWorld::claim_spirit(Origin::signed(BOB), 1, overlord_signature, metadata));
	});
}

#[test]
fn claimed_spirit_twice_fails() {
	ExtBuilder::default().build(ALICE).execute_with(|| {
		let overlord_pair = sr25519::Pair::from_seed(b"09876543210987654321098765432109");
		//let overlord_pub = overlord_pair.public();
		let metadata = stb("I am Overlord");
		let claim = Encode::encode(&(ALICE, metadata.clone()));
		let overlord_signature = overlord_pair.sign(&claim);
		// Set the Overlord Admin account
		assert_ok!(PhalaWorld::set_overlord(Origin::root(), BOB));
		//  Only root can set the Overlord Admin account
		assert_noop!(PhalaWorld::set_overlord(Origin::signed(ALICE), BOB), BadOrigin);
		// Last Event Overlord changed from None
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::OverlordChanged {
			old_overlord: Some(ALICE),
		}));
		// Mint Spirits collection
		mint_collection(BOB);
		// Enable spirits to be claimed
		assert_ok!(PhalaWorld::set_status_type(
			Origin::signed(BOB),
			true,
			StatusType::ClaimSpirits
		));
		// Dispatch a claim spirit from ALICE's account
		assert_ok!(PhalaWorld::claim_spirit(
			Origin::signed(ALICE),
			1,
			overlord_signature.clone(),
			metadata.clone()
		));
		// Fail to dispatch a second claim spirit
		assert_noop!(
			PhalaWorld::claim_spirit(Origin::signed(ALICE), 1, overlord_signature, metadata),
			Error::<Test>::SpiritAlreadyClaimed
		);
	});
}

#[test]
fn start_world_clock_works() {
	ExtBuilder::default().build(ALICE).execute_with(|| {
		// Set the Overlord Admin account
		assert_ok!(PhalaWorld::set_overlord(Origin::root(), ALICE));
		// Initialize the Phala World Clock
		assert_ok!(PhalaWorld::initialize_world_clock(Origin::signed(ALICE)));
	});
}

#[test]
fn auto_increment_era_works() {
	ExtBuilder::default().build(ALICE).execute_with(|| {
		// Set Overlord admin as BOB
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
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::NewEra { time: 6, era: 1 }));
	});
}

#[test]
fn preorder_egg_works() {
	ExtBuilder::default().build(ALICE).execute_with(|| {
		let mut alice_metadata = BoundedVec::default();
		let mut bob_metadata = BoundedVec::default();
		let mut charlie_metadata = BoundedVec::default();
		metadata_accounts(alice_metadata.clone(), bob_metadata.clone(), charlie_metadata.clone());
		// Set the Overlord Admin account
		assert_ok!(PhalaWorld::set_overlord(Origin::root(), ALICE));
		// Enable preorder of eggs
		assert_ok!(PhalaWorld::set_status_type(
			Origin::signed(ALICE),
			true,
			StatusType::PreorderEggs
		));
		// BOB preorders an egg
		assert_ok!(PhalaWorld::preorder_egg(
			Origin::signed(BOB),
			RaceType::Cyborg,
			CareerType::HardwareDruid,
			bob_metadata
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::EggPreordered {
			owner: BOB,
			preorder_id: 0,
		}));
		// ALICE preorders an egg
		assert_ok!(PhalaWorld::preorder_egg(
			Origin::signed(ALICE),
			RaceType::Pandroid,
			CareerType::HardwareDruid,
			alice_metadata
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::EggPreordered {
			owner: ALICE,
			preorder_id: 1,
		}));
		// CHARLIE fails to preorder an egg with CareerType HardwareDruid
		assert_noop!(
			PhalaWorld::preorder_egg(
				Origin::signed(CHARLIE),
				RaceType::Cyborg,
				CareerType::HardwareDruid,
				charlie_metadata.clone()
			),
			Error::<Test>::CareerMintMaxReached
		);
		// Reassign PreorderIndex to max value
		PreorderIndex::<Test>::mutate(|id| *id = PreorderId::max_value());
		// CHARLIE preorders an egg but max value is reached
		assert_noop!(
			PhalaWorld::preorder_egg(
				Origin::signed(CHARLIE),
				RaceType::Cyborg,
				CareerType::HackerWizard,
				charlie_metadata
			),
			Error::<Test>::NoAvailablePreorderId
		);
	});
}

#[test]
fn preorder_egg_works_2() {
	ExtBuilder::default().build(ALICE).execute_with(|| {
		let mut alice_metadata = BoundedVec::default();
		let mut bob_metadata = BoundedVec::default();
		let mut charlie_metadata = BoundedVec::default();
		metadata_accounts(alice_metadata.clone(), bob_metadata.clone(), charlie_metadata.clone());
		// Set the Overlord Admin account
		assert_ok!(PhalaWorld::set_overlord(Origin::root(), ALICE));
		// Enable preorder of eggs
		assert_ok!(PhalaWorld::set_status_type(
			Origin::signed(ALICE),
			true,
			StatusType::PreorderEggs
		));
		// BOB preorders an egg
		assert_ok!(PhalaWorld::preorder_egg(
			Origin::signed(BOB),
			RaceType::Cyborg,
			CareerType::HardwareDruid,
			bob_metadata
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::EggPreordered {
			owner: BOB,
			preorder_id: 0,
		}));
		// ALICE preorders an egg
		assert_ok!(PhalaWorld::preorder_egg(
			Origin::signed(ALICE),
			RaceType::Cyborg,
			CareerType::HardwareDruid,
			alice_metadata
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::EggPreordered {
			owner: ALICE,
			preorder_id: 1,
		}));
		// CHARLIE fails to preorder an egg with CareerType HardwareDruid
		assert_noop!(
			PhalaWorld::preorder_egg(
				Origin::signed(CHARLIE),
				RaceType::Cyborg,
				CareerType::HackerWizard,
				charlie_metadata.clone()
			),
			Error::<Test>::RaceMintMaxReached
		);
		// Reassign PreorderIndex to max value
		PreorderIndex::<Test>::mutate(|id| *id = PreorderId::max_value());
		// CHARLIE preorders an egg but max value is reached
		assert_noop!(
			PhalaWorld::preorder_egg(
				Origin::signed(CHARLIE),
				RaceType::Pandroid,
				CareerType::HackerWizard,
				charlie_metadata
			),
			Error::<Test>::NoAvailablePreorderId
		);
	});
}

#[test]
fn mint_preorder_egg_works() {
	ExtBuilder::default().build(ALICE).execute_with(|| {
		let mut alice_metadata = BoundedVec::default();
		let mut bob_metadata = BoundedVec::default();
		let mut charlie_metadata = BoundedVec::default();
		metadata_accounts(alice_metadata.clone(), bob_metadata.clone(), charlie_metadata.clone());
		// Set the Overlord Admin account
		assert_ok!(PhalaWorld::set_overlord(Origin::root(), ALICE));
		// Alice mints collection for Spirits
		mint_collection(ALICE);
		// Alice mints collection for Eggs
		mint_collection(ALICE);
		// Enable preorder of eggs
		assert_ok!(PhalaWorld::set_status_type(
			Origin::signed(ALICE),
			true,
			StatusType::PreorderEggs
		));
		// BOB preorders an egg
		assert_ok!(PhalaWorld::preorder_egg(
			Origin::signed(BOB),
			RaceType::Cyborg,
			CareerType::HardwareDruid,
			bob_metadata
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::EggPreordered {
			owner: BOB,
			preorder_id: 0,
		}));
		// CHARLIE preorders an egg
		assert_ok!(PhalaWorld::preorder_egg(
			Origin::signed(CHARLIE),
			RaceType::Pandroid,
			CareerType::HardwareDruid,
			charlie_metadata
		));
		// Check if event triggered
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::EggPreordered {
			owner: CHARLIE,
			preorder_id: 1,
		}));
		// ALICE fails to preorder an egg with CareerType HardwareDruid
		assert_noop!(
			PhalaWorld::preorder_egg(
				Origin::signed(ALICE),
				RaceType::Cyborg,
				CareerType::HardwareDruid,
				alice_metadata.clone()
			),
			Error::<Test>::CareerMintMaxReached
		);
		// Reassign PreorderIndex to max value
		PreorderIndex::<Test>::mutate(|id| *id = PreorderId::max_value());
		// ALICE preorders an egg but max value is reached
		assert_noop!(
			PhalaWorld::preorder_egg(
				Origin::signed(ALICE),
				RaceType::Cyborg,
				CareerType::HackerWizard,
				alice_metadata
			),
			Error::<Test>::NoAvailablePreorderId
		);
		// Overlord mints eggs
		assert_ok!(PhalaWorld::mint_eggs(Origin::signed(ALICE)));
		// Check if event triggered
		System::assert_last_event(MockEvent::PhalaWorld(crate::Event::EggMinted {
			collection_id: 1,
			nft_id: 1,
			owner: BOB,
		}));
		// Check Balances of ALICE and BOB
		assert_eq!(Balances::total_balance(&ALICE), 20_000_020 * PHA);
		assert_eq!(Balances::total_balance(&BOB), 14_990 * PHA);
		assert_eq!(Balances::total_balance(&CHARLIE), 149_990 * PHA);
	});
}
