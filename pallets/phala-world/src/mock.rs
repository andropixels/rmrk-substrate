use super::*;
use crate as pallet_phala_world;

use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU32, ConstU64, Everything, GenesisBuild, OnFinalize, OnInitialize},
	weights::Weight,
};
use frame_system as system;
use frame_system::EnsureRoot;
use sp_core::{crypto::AccountId32, sr25519::Signature, Pair, Public, H256};

use rmrk_traits::AccountIdOrCollectionNftTuple;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type AccountId = AccountId32;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;
type BlockNumber = u64;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Uniques: pallet_uniques::{Pallet, Storage, Event<T>},
		RmrkCore: pallet_rmrk_core::{Pallet, Call, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		RmrkMarket: pallet_rmrk_market::{Pallet, Call, Event<T>},
		PhalaWorld: pallet_phala_world::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
}

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<2>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub ClassBondAmount: Balance = 100;
	pub MaxMetadataLength: u32 = 256;
	pub const MaxRecursions: u32 = 10;
	pub const ResourceSymbolLimit: u32 = 10;
}

impl pallet_rmrk_core::Config for Test {
	type Event = Event;
	type ProtocolOrigin = EnsureRoot<AccountId>;
	type MaxRecursions = MaxRecursions;
	type ResourceSymbolLimit = ResourceSymbolLimit;
}

parameter_types! {
	pub const ClassDeposit: Balance = 10_000 * PHA; // 1 UNIT deposit to create asset class
	pub const InstanceDeposit: Balance = 100 * PHA; // 1/100 UNIT deposit to create asset instance
	pub const KeyLimit: u32 = 32;	// Max 32 bytes per key
	pub const ValueLimit: u32 = 64;	// Max 64 bytes per value
	pub const UniquesMetadataDepositBase: Balance = 1000 * PHA;
	pub const AttributeDepositBase: Balance = 100 * PHA;
	pub const DepositPerByte: Balance = 10 * PHA;
	pub const UniquesStringLimit: u32 = 32;
}

impl pallet_uniques::Config for Test {
	type Event = Event;
	type ClassId = u32;
	type InstanceId = u32;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type ClassDeposit = ClassDeposit;
	type InstanceDeposit = InstanceDeposit;
	type MetadataDepositBase = UniquesMetadataDepositBase;
	type AttributeDepositBase = AttributeDepositBase;
	type DepositPerByte = DepositPerByte;
	type StringLimit = UniquesStringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = ();
	// type InstanceReserveStrategy = NFT;
}

parameter_types! {
	pub const MinimumOfferAmount: Balance = 50 * UNITS;
}

impl pallet_rmrk_market::Config for Test {
	type Event = Event;
	type ProtocolOrigin = EnsureRoot<AccountId>;
	type Currency = Balances;
	type MinimumOfferAmount = MinimumOfferAmount;
}

parameter_types! {
	pub const BlocksPerEra: BlockNumber = 5;
	pub const FounderEggPrice: Balance = 1_000 * PHA;
	pub const LegendaryEggPrice: Balance = 100 * PHA;
	pub const NormalEggPrice: Balance = 10 * PHA;
	pub const MaxMintPerRace: u32 = 2;
	pub const MaxMintPerCareer: u32 = 2;
}

impl Config for Test {
	type Event = Event;
	type OverlordOrigin = EnsureRoot<AccountId>;
	type Currency = Balances;
	type BlocksPerEra = BlocksPerEra;
	type FounderEggPrice = FounderEggPrice;
	type LegendaryEggPrice = LegendaryEggPrice;
	type NormalEggPrice = NormalEggPrice;
	type MaxMintPerRace = MaxMintPerRace;
	type MaxMintPerCareer = MaxMintPerCareer;
}

pub type SystemCall = frame_system::Call<Test>;
pub type BalanceCall = pallet_balances::Call<Test>;

pub fn fast_forward_to(n: u64) {
	while System::block_number() < n {
		System::set_block_number(System::block_number() + 1);
		System::on_finalize(System::block_number());
		PhalaWorld::on_finalize(System::block_number());
	}
}

pub const ALICE: AccountId = AccountId::new([
	116, 28, 8, 160, 111, 65, 197, 150, 96, 143, 103, 116, 37, 155, 217, 4, 51, 4, 173, 250, 93,
	62, 234, 98, 118, 11, 217, 190, 151, 99, 77, 99,
]);
pub const BOB: AccountId = AccountId::new([
	250, 140, 153, 155, 88, 13, 83, 23, 193, 161, 236, 241, 58, 213, 107, 213, 230, 33, 38, 154,
	78, 125, 67, 186, 54, 157, 62, 131, 179, 150, 232, 82,
]);
pub const CHARLIE: AccountId = AccountId::new([3u8; 32]);
pub const PHA: Balance = 1;
pub const UNITS: Balance = 100_000_000_000;

pub const MILLISECS_PER_BLOCK: u64 = 3_000;
// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

pub struct ExtBuilder;

impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder
	}
}
// Build genesis storage according to the mock runtime.
impl ExtBuilder {
	pub fn build(self, overlord_key: AccountId32) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		pallet_phala_world::GenesisConfig::<Test> {
			zero_day: None,
			overlord: Some(overlord_key),
			era: 0,
			can_claim_spirits: false,
			can_purchase_rare_eggs: false,
			can_preorder_eggs: false,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_balances::GenesisConfig::<Test> {
			balances: vec![
				(ALICE, 20_000_000 * PHA),
				(BOB, 15_000 * PHA),
				(CHARLIE, 150_000 * PHA),
			],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
