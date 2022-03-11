use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RaceType {
	Cyborg,
	AISpectre,
	XGene,
	Pandroid,
}
