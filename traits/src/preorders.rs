use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::cmp::Eq;

use crate::{career::CareerType, race::RaceType};
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PreorderInfo<AccountId> {
	/// Account owner of the Egg preorder
	pub owner: AccountId,
	/// Race type of the preorder
	pub race: RaceType,
	/// Career type of the preorder
	pub career: CareerType,
}
