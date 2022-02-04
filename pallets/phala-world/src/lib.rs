#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::DispatchResult, ensure, traits::{Currency, tokens::nonfungibles::*}, transactional, BoundedVec,
};
use frame_system::ensure_signed;

use sp_std::prelude::*;
use sp_std::result::Result;

pub use pallet_rmrk_core::types::*;

use rmrk_traits::{
	EggInfo,
	world_clock::WorldClock,
	primitives::*,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use pallet::*;

/// Constant for amount of time it takes for an Egg to hatch after hatching is started
pub const HATCHING_DURATION: u128 = 1_000_000;

// Egg Types of Normal, Legendary & Founder
//#[derive(Encode, Decode, Copy, Clone, PartialEq)]
//pub enum EggType {
//	Normal = 0,
//	Legendary = 1,
//	Founder = 2,
//}

//impl Default for EggType {
//	fn default() -> Self {
//		EggType::Normal
//	}
//}

//impl EggType {
//	pub fn from_u8(value: u8) -> EggType {
//		match value {
//			0 => EggType::Normal,
//			1 => EggType::Legendary,
//			2 => EggType::Founder,
//			_ => EggType::Normal,
//		}
//	}
//}

// Four Races to choose from
//#[derive(Encode, Decode, Clone, PartialEq)]
//pub enum RaceType {
//	Cyborg = 0,
//	PhatrixAmrita = 1,
//	Devil = 2,
//	Robot = 3,
//}

// Five Careers to choose from
//#[derive(Encode, Decode, Clone, PartialEq)]
//pub enum CareerType {
//	HardwareDruid = 0,
//	RoboWarrior = 1,
//	NegotiateTrade = 2,
//	HackerWizard = 3,
//	Web3Monk = 4,
//}

// TODO: Make this a trait w/ function to start the world clock & auto update eras

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use rmrk_traits::Nft;
	use rmrk_traits::primitives::{CollectionId, NftId, SerialId};
	use crate::{CareerType, EggType, RaceType};

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_rmrk_core::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The origin which may forcibly buy, sell, list/unlist, offer & withdraw offer on Tokens
		type ProtocolOrigin: EnsureOrigin<Self::Origin>;
		/// The market currency mechanism.
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Stores all of the valid claimed spirits from the airdrop by serial id & bool true if claimed
	#[pallet::storage]
	#[pallet::getter(fn claimed_spirits)]
	pub type ClaimedSpirits<T: Config> = StorageMap<_, Twox64Concat, SerialId, bool>;

	/// Stores all of the valid claimed Eggs from the whitelist or preorder
	#[pallet::storage]
	#[pallet::getter(fn claimed_eggs)]
	pub type ClaimedEggs<T: Config> = StorageMap<_, Twox64Concat, SerialId, bool>;

	/// Food per Owner where an owner gets 5 food per era
	#[pallet::storage]
	#[pallet::getter(fn get_food_by_owner)]
	pub type FoodByOwner<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u8>;

	// TODO: Era index for Phala World
	//#[pallet::storage]
	//#[pallet::getter(fn get_next_era)]
	//pub type EraIndex<T:Config> = StorageValue<_, , ValueQuery>;

	// The pallet's runtime storage items.
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Phala World clock zero day started
		WorldClockStarted {
			start_time: T::BlockNumber,
			era: u128,
		},
		/// Start of a new era
		NewEra {
			time: T::BlockNumber,
			era: u128,
		},
		/// Spirit has been claimed from the whitelist
		SpiritClaimed {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
		},
		/// Founder egg has been purchased
		FounderEggPurchased {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
		},
		/// Legendary egg has been purchased
		LegendaryEggPurchased {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
		},
		/// A chance to get an egg through preorder
		EggPreordered {
			price: BalanceOf<T>,
			owner: T::AccountId,
		},
		/// Egg claimed from the winning preorder
		EggClaimed {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
		},
		/// Refund claimed by owner that did not win an Egg
		RefundClaimed {
			price: BalanceOf<T>,
			owner: T::AccountId,
		},
		/// Egg received food from an account
		EggFoodReceived {
			collection_id: CollectionId,
			nft_id: NftId,
			sender: T::AccountId,
			owner: T::AccountId,
		},
		/// Egg owner has initiated the hatching sequence
		StartedHatching {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
		},
		/// A top 10 fed egg of the era has updated their hatch time
		HatchTimeUpdated {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
			hatch_time: T::BlockNumber,
		},
		/// An egg has been hatched
		EggHatched {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
		},
		/// Shell has been awakened from an egg being hatched and burned
		ShellAwakened {
			collection_id: CollectionId,
			nft_id: NftId,
			owner: T::AccountId,
			career: u8,
			race: u8,
		},
		/// Egg hatching has been disabled & no other eggs can be hatched
		EggHatchingDisabled {
			collection_id: CollectionId,
			can_hatch: bool,
		},
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		AccountNotInWhitelist,
		NoClaimAvailable,
		SpiritAlreadyClaimed,
		ClaimIsOver,
		InsufficientFunds,
		InvalidClaimTicket,
		CannotHatchEgg,
		CannotSendFoodToEgg,
		NoFoodAvailable,
		NoPermission,
		CareerAndRaceAlreadyChosen,
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Claim a spirit for users that are on the whitelist. This whitelist will consist of a
		/// a serial id and an account id that is signed by the admin account. When a user comes
		/// to claim their spirit, they will provide a serial id & will be validated as an
		/// authenticated claimer
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic.
		/// - serial_id: The serial id of the spirit to be claimed.
		/// - signature: The signature of the account that is claiming the spirit.
		/// - metadata: The metadata of the account that is claiming the spirit.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn claim_spirit(
			origin: OriginFor<T>,
			serial_id: SerialId,
			signature: BoundedVec<u8, T::StringLimit>, // TODO: change to Signature
			metadata: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// Buy a rare egg of either type Legendary or Founder. Both Egg types will have a set
		/// price. These will also be limited in quantity and on a first come, first serve basis.
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic.
		/// - egg_type: The type of egg to be purchased.
		/// - race: The race of the egg chosen by the user.
		/// - career: The career of the egg chosen by the user or auto-generated based on metadata
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn buy_rare_egg(
			origin: OriginFor<T>,
			egg_type: EggType,
			race: RaceType,
			career: CareerType,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// Users can pre-order an egg. This will enable users that are whitelisted to be
		/// added to the queue of users that can claim eggs. Those that come after the whitelist
		/// pre-sale will be able to win the chance to acquire an egg based on their choice of
		/// race and career as they will have a limited quantity.
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic preordering the egg
		/// - race: The race that the user has chosen (limited # of races)
		/// - career: The career that the user has chosen (limited # of careers)
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn preorder_egg(
			origin: OriginFor<T>,
			race: RaceType,
			career: CareerType,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// This is an admin only function that will be called to do a bulk minting of all egg
		/// owners that made selected a race and career that was available based on the quantity
		/// available. Those that did not win an egg will have to claim their refund
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn mint_eggs(
			origin: OriginFor<T>,
			egg_owners: Vec<SerialId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// For users that did not win an egg, they can claim their refund here and will be
		/// given the amount the paid to preorder an egg deposited to their account
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic claiming the refund
		/// - claim_id: The serial id of the claim that is being claimed
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn claim_refund(
			origin: OriginFor<T>,
			claim_id: SerialId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// Once users have received their eggs and the start hatching event has been triggered,
		/// they can start the hatching process and a timer will start for the egg to hatch at
		/// a designated time. Eggs can reduce their time by being in the top 10 of egg's fed
		/// per era.
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic starting the hatching process
		/// - collection_id: The collection id of the Egg RMRK NFT
		/// - nft_id: The NFT id of the Egg RMRK NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn start_hatching(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// Feed another egg to the current egg being hatched. This will reduce the time left to
		/// hatching if the egg is in the top 10 of eggs fed that era.
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic feeding the egg
		/// - collection_id: The collection id of the Egg RMRK NFT
		/// - nft_id: The NFT id of the Egg RMRK NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn feed_egg(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// Hatch the egg that is currently being hatched. This will trigger the end of the hatching
		/// process and the egg will be burned. After burning, the user will receive the awakened
		/// Shell RMRK NFT
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic hatching the egg
		/// - collection_id: The collection id of the Egg RMRK NFT
		/// - nft_id: The NFT id of the Egg RMRK NFT
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn hatch_egg(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// This is an admin function to update eggs hatch times based on being in the top 10 of
		/// fed eggs within that era
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic updating the eggs hatch times
		/// - collection_id: The collection id of the Egg RMRK NFT
		/// - nft_id: The NFT id of the Egg RMRK NFT
		/// - reduced_time: The amount of time the egg will be reduced by
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn update_hatch_time(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: NftId,
			reduced_time: u64,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
