use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use frame_support::pallet_prelude::*;

#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CareerType {
	HardwareDruid,
	RoboWarrior,
	TradeNegotiator,
	HackerWizard,
	Web3Monk,
}
