#![cfg_attr(not(feature = "std"), no_std)]

pub mod base;
pub mod collection;
pub mod nft;
pub mod part;
pub mod priority;
pub mod property;
pub mod resource;
pub mod theme;
pub mod egg;
pub mod preorders;
pub mod status_type;

pub use base::{Base, BaseInfo};
pub use part::{EquippableList, FixedPart, PartType, SlotPart};
pub use theme::{Theme, ThemeProperty};
// pub use part::{PartInfo};
pub use collection::{Collection, CollectionInfo};
pub use nft::{AccountIdOrCollectionNftTuple, Nft, NftInfo};
pub use priority::Priority;
pub use property::Property;
pub use resource::{Resource, ResourceInfo};
pub use egg::{Egg, EggInfo};
pub use preorders::PreorderInfo;

pub mod primitives {
	pub type CollectionId = u32;
	pub type ResourceId = u32;
	pub type NftId = u32;
	pub type BaseId = u32;
	pub type SlotId = u32;
	pub type PartId = u32;
	pub type ZIndex = u32;
	pub type SerialId = u32;
	pub type EraId = u128;
	pub type RaceType = u8;
	pub type CareerType = u8;
}
