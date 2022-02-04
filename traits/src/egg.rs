use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::cmp::Eq;

use crate::primitives::*;
use serde::{Deserialize, Serialize};
use sp_std::result::Result;

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct EggInfo<SerialId, CollectionId, NftId> {
    /// Egg id of the Egg RMRK NFT
    pub egg_id: SerialId,
    /// Egg type of the Egg RMRK NFT
    pub egg_type: EggType,
    /// Collection id of the Egg RMRK NFT
    pub collection_id: CollectionId,
    /// NFT id of the Egg RMRK NFT
    pub nft_id: NftId,
    /// Race type of the Egg RMRK NFT
    pub race: u8,
    /// Career type of the Egg RMRK NFT
    pub career: u8,
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