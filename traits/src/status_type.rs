use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Encode, Decode, Debug, Clone, PartialEq, TypeInfo)]
pub enum StatusType {
    ClaimSpirits,
    PurchaseRareEggs,
    PreorderEggs,
}

#[allow(clippy::upper_case_acronyms)]
impl StatusType {
    pub fn from_u8(value: u8) -> Option<StatusType> {
        match value {
            0 => Some(StatusType::ClaimSpirits),
            1 => Some(StatusType::PurchaseRareEggs),
            2 => Some(StatusType::PreorderEggs),
            _ => None,
        }
    }
}