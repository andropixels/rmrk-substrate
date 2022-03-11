use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, RuntimeDebug};
use sp_std::cmp::Eq;

use crate::{career::CareerType, primitives::*, race::RaceType};
use serde::{Deserialize, Serialize};
use sp_std::result::Result;

// Egg Types of Normal, Legendary & Founder
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum EggType {
	Normal,
	Legendary,
	Founder,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct EggInfo {
	/// Egg type of the Egg RMRK NFT
	pub egg_type: EggType,
	/// Race type of the Egg RMRK NFT
	pub race: RaceType,
	/// Career type of the Egg RMRK NFT
	pub career: CareerType,
	/// Block number when the Egg started hatching process
	pub start_hatching: u64,
	/// Time duration from `start_hatching` to when the Egg is ready to hatch
	/// 0 if the Egg has not started the hatching process
	pub hatching_duration: u64,
}

pub trait Egg<AccountId, CollectionId, NftId, BlockNumber> {
	/// When a user initiates the hatching process, this function will set the start time for the
	/// hatching process.
	fn set_start_hatch_time(
		sender: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
	) -> Result<BlockNumber, DispatchError>;
	/// Get the `hatching_duration` of the Egg RMRK NFT and reduce it by `reduce_time_by`
	/// This will be executed by the admin account
	fn update_hatch_time(
		admin: AccountId,
		collection_id: CollectionId,
		nft_id: NftId,
		reduce_time_by: u64,
	) -> Result<BlockNumber, DispatchError>;
}
