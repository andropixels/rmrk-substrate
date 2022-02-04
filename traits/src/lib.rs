#![cfg_attr(not(feature = "std"), no_std)]

pub mod collection;
pub mod nft;
pub mod priority;
pub mod property;
pub mod resource;
pub mod egg;
pub mod world_clock;

pub use collection::{Collection, CollectionInfo};
pub use nft::{AccountIdOrCollectionNftTuple, Nft, NftInfo};
pub use priority::Priority;
pub use property::Property;
pub use resource::{Resource, ResourceInfo};
pub use egg::EggInfo;
use world_clock::{WorldClock, WorldClockInfo};

pub mod primitives {
	pub type CollectionId = u32;
	pub type ResourceId = u32;
	pub type NftId = u32;
	pub type SerialId = u32;
	pub type EraId = u128;
	pub type EggType = u8;
	pub type RaceType = u8;
	pub type CareerType = u8;
}
