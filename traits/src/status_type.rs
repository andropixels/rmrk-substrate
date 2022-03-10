use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Encode, Decode, Debug, Clone, PartialEq, TypeInfo)]
pub enum StatusType {
	ClaimSpirits,
	PurchaseRareEggs,
	PreorderEggs,
}
