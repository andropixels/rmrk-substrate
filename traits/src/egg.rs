use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::cmp::Eq;

use crate::primitives::*;
use serde::{Deserialize, Serialize};
use sp_std::result::Result;

// Egg Types of Normal, Legendary & Founder
// #[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
// pub enum EggType {
// 	Normal = 0,
// 	Legendary = 1,
// 	Founder = 2,
// }
//
// impl Default for EggType {
// 	fn default() -> Self {
// 		EggType::Normal
// 	}
// }
//
// impl EggType {
// 	pub fn from_u8(value: u8) -> EggType {
// 		match value {
// 			0 => EggType::Normal,
// 			1 => EggType::Legendary,
// 			2 => EggType::Founder,
// 			_ => EggType::Normal,
// 		}
// 	}
// }
//
// // Four Races to choose from
// #[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
// pub enum RaceType {
// 	Cyborg = 0,
// 	AI = 1,
// 	Devil = 2,
// 	Robot = 3,
// }
//
// impl RaceType {
//     pub fn from_u8(value: u8) -> RaceType {
//         match value {
//             0 => RaceType::Cyborg,
//             1 => RaceType::AI,
//             2 => RaceType::Devil,
//             3 => RaceType::Robot,
//         }
//     }
// }
//
// // Five Careers to choose from
// #[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
// pub enum CareerType {
// 	HardwareDruid = 0,
// 	RoboWarrior = 1,
// 	TradeNegotiator = 2,
// 	HackerWizard = 3,
// 	Web3Monk = 4,
// }
//
// impl CareerType {
//     pub fn from_u8(value: u8) -> CareerType {
//         match value {
//             0 => CareerType::HardwareDruid,
//             1 => CareerType::RoboWarrior,
//             2 => CareerType::TradeNegotiator,
//             3 => CareerType::HackerWizard,
//             4 => CareerType::Web3Monk,
//         }
//     }
// }

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct EggInfo<CollectionId, NftId> {
    /// Egg type of the Egg RMRK NFT
    pub egg_type: EggType,
    /// Collection id of the Egg RMRK NFT
    pub collection_id: CollectionId,
    /// NFT id of the Egg RMRK NFT
    pub nft_id: NftId,
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