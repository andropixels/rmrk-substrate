use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::cmp::Eq;

use crate::primitives::*;
use serde::{Deserialize, Serialize};
use sp_std::result::Result;

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PreorderInfo<AccountId> {
    /// Account owner of the Egg preorder
    pub owner: AccountId,
    /// Egg type of the preorder
    pub egg_type: EggType,
    /// Race type of the preorder
    pub race: RaceType,
    /// Career type of the preorder
    pub career: CareerType,
}