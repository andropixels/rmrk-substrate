#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// Constant for amount of time it takes for an Egg to hatch after hatching is started
pub const HATCHING_DURATION: u128 = 1_000_000;

/// Egg Types of Normal, Legendary & Founder
#[derive(Encode, Decode, Copy, Clone, PartialEq)]
pub enum EggType {
	Normal = 0,
	Legendary = 1,
	Founder = 2,
}

impl Default for EggType {
	fn default() -> Self {
		EggType::Normal
	}
}

impl EggType {
	pub fn from_u8(value: u8) -> EggType {
		match value {
			0 => EggType::Normal,
			1 => EggType::Legendary,
			2 => EggType::Founder,
			_ => EggType::Normal,
		}
	}
}

/// Four Races to choose from
#[derive(Encode, Decode, Clone, PartialEq)]
pub enum RaceType {
	Cyborg = 0,
	PhatrixAmrita = 1,
	Devil = 2,
	Robot = 3,
}

/// Five Careers to choose from
#[derive(Encode, Decode, Clone, PartialEq)]
pub enum CareerType {
	HardwareDruid = 0,
	RoboWarrior = 1,
	NegotiateTrade = 2,
	HackerWizard = 3,
	Web3Monk = 4,
}

// TODO: Make this a trait w/ function to start the world clock & auto update eras

/// Phala World Clock
pub struct WorldClockInfo<BlockNumber> {
	/// Zero Day of Phala World
	zero_day: BlockNumber,
	/// Current number of eras
	eras: u128,
}

// TODO: Make this a trait w/ functions to update hatching duration & get time left to hatch

/// Hatch info for Eggs
pub struct HatchEggInfo<CollectionId, NftId, BlockNumber> {
	/// Collection Id of the Egg
	collection_id: CollectionId,
	/// Nft Id of the Egg
	nft_id: NftId,
	/// Start hatching event block number
	start_hatching: BlockNumber,
	/// Time duration for hatching
	hatching_duration: u128,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use rmrk_traits::Nft;
	use rmrk_traits::primitives::{CollectionId, NftId};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Stores all of the valid claimed spirits from the airdrop by serial id & bool true if claimed
	#[pallet::storage]
	#[pallet::getter(fn claimed_spirits)]
	pub type ClaimedSpirits<T: Config> = StorageMap<_, Twox64Concat, u64, bool>;

	/// Stores all of the valid claimed Eggs from the whitelist or preorder
	#[pallet::storage]
	#[pallet::getter(fn claimed_spirits)]
	pub type ClaimedEggs<T: Config> = StorageMap<_, Twox64Concat, u64, bool>;

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
			price: T::Balance,
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
			price: T::Balance,
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
