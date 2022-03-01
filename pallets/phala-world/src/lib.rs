#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	ensure, traits::Currency, transactional, BoundedVec,
};
use frame_system::ensure_signed;

use sp_std::prelude::*;
use sp_std::result::Result;

pub use pallet_rmrk_core::types::*;
pub use pallet_rmrk_market;

use rmrk_traits::{
	EggInfo,
	PreorderInfo,
	primitives::*,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use pallet::*;

/// Spirit Collection ID
pub const SPIRIT_COLLECTION_ID: u32 = 0;
/// Constant for Collection ID for Eggs
pub const EGGS_COLLECTION_ID: u32 = 1;
/// Constant for amount of time it takes for an Egg to hatch after hatching is started
pub const HATCHING_DURATION: u64 = 1_000_000;
/// Constant for Founder Eggs Price
pub const FOUNDER_EGG_PRICE: u128 = 1_000_000;
/// Constant for Legendary Eggs Price
pub const LEGENDARY_EGG_PRICE: u128 = 100_000;
/// Constant for Normal Eggs Price
pub const NORMAL_EGG_PRICE: u128 = 1_000;

// Egg Types of Normal, Legendary & Founder
// #[derive(Encode, Decode, Copy, Clone, PartialEq)]
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

// Four Races to choose from
//#[derive(Encode, Decode, Clone, PartialEq)]
//pub enum RaceType {
//	Cyborg = 0,
//	AISpectre = 1,
//	XGene = 2,
//	Pandroid = 3,
//}

// Five Careers to choose from
//#[derive(Encode, Decode, Clone, PartialEq)]
//pub enum CareerType {
//	HardwareDruid = 0,
//	RoboWarrior = 1,
//	TradeNegotiator = 2,
//	HackerWizard = 3,
//	Web3Monk = 4,
//}
pub enum StatusType {
	ClaimSpirits = 0,
	PurchaseRareEggs = 1,
	PreorderEggs = 2,
}

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

// #[cfg(feature = "std")]
// use serde::{Deserialize, Serialize};
//
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize, PartialEq, Eq))]
// #[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone)]
// pub struct OverlordInfo<AccountId> {
// 	pub admin: AccountId,
// 	pub collection_id: u32,
// }

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::StaticLookup;
	use frame_support::sp_runtime::traits::Zero;
	use frame_system::Origin;
	use rmrk_traits::Nft;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	type PreorderInfoOf<T> = PreorderInfo<<T as frame_system::Config>::AccountId>;
	//type OverlordInfoOf<T> = OverlordInfo<<T as frame_system::Config>::AccountId>;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_rmrk_core::Config + pallet_rmrk_market::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The origin which may forcibly buy, sell, list/unlist, offer & withdraw offer on Tokens
		type OverlordOrigin: EnsureOrigin<Self::Origin>;
		/// The market currency mechanism.
		type Currency: Currency<Self::AccountId>;
		/// Block per Era that will increment the Era storage value every interval
		#[pallet::constant]
		type BlocksPerEra: Get<Self::BlockNumber>;
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

	/// Preorder info vector for Accounts
	#[pallet::storage]
	#[pallet::getter(fn preorders)]
	pub type Preorders<T: Config> = StorageValue<_, Vec<PreorderInfoOf<T>>, ValueQuery>;

	/// Stores all the Eggs and the information about the Egg pertaining to Hatch times and feeding
	#[pallet::storage]
	#[pallet::getter(fn eggs)]
	pub type Eggs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CollectionId,
		Blake2_128Concat,
		NftId,
		EggInfo<CollectionId, NftId>,
	>;

	/// Food per Owner where an owner gets 5 food per era
	#[pallet::storage]
	#[pallet::getter(fn get_food_by_owner)]
	pub type FoodByOwner<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u8>;

	/// Phala World Zero Day `BlockNumber` this will be used to determine Eras
	#[pallet::storage]
	#[pallet::getter(fn zero_day)]
	pub(super) type ZeroDay<T:Config> = StorageValue<_, T::BlockNumber, OptionQuery>;

	/// The current Era from the initial ZeroDay BlockNumber
	#[pallet::storage]
	#[pallet::getter(fn era)]
	pub type Era<T:Config> = StorageValue<_, u64, ValueQuery>;

	/// Spirits can be claimed
	#[pallet::storage]
	#[pallet::getter(fn can_claim_spirits)]
	pub type CanClaimSpirits<T:Config> = StorageValue<_, bool, ValueQuery>;

	/// Rare Eggs can be purchased
	#[pallet::storage]
	#[pallet::getter(fn can_purchase_rare_eggs)]
	pub type CanPurchaseRareEggs<T:Config> = StorageValue<_, bool, ValueQuery>;

	/// Eggs can be preordered
	#[pallet::storage]
	#[pallet::getter(fn can_preorder_eggs)]
	pub type CanPreorderEggs<T:Config> = StorageValue<_, bool, ValueQuery>;

	/// Overlord Admin account of Phala World
	#[pallet::storage]
	#[pallet::getter(fn overlord)]
	pub(super) type Overlord<T:Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: T::BlockNumber) {
			if let Some(zero_day) = <ZeroDay<T>>::get() {
				let blocks_since_zero_day = n - zero_day;
				if (blocks_since_zero_day % T::BlocksPerEra::get()).is_zero() {
					let mut current_era = <Era<T>>::get();
					current_era = current_era.saturating_add(1u64);
					<Era<T>>::put(current_era);
					Self::deposit_event(Event::NewEra{
						time: n,
						era: current_era,
					});
				}
			}
		}
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		/// `BlockNumber` of Phala World Zero Day
		pub zero_day: Option<T::BlockNumber>,
		/// Overlord Admin account of Phala World
		pub overlord: Option<T::AccountId>,
		/// Current Era of Phala World
		pub era: u64,
		/// bool for if a Spirit is claimable
		pub can_claim_spirits: bool,
		/// bool for if a Rare Egg can be purchased
		pub can_purchase_rare_eggs: bool,
		/// bool for if an Egg can be preordered
		pub can_preorder_eggs: bool,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				zero_day: None,
				overlord: None,
				era: 0,
				can_claim_spirits: false,
				can_purchase_rare_eggs: false,
				can_preorder_eggs: false,
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(ref zero_day) = self.zero_day {
				<ZeroDay<T>>::put(zero_day);
			}
			if let Some(ref overlord) = self.overlord {
				<Overlord<T>>::put(overlord);
			}
			let era = self.era;
			<Era<T>>::put(era);
			let can_claim_spirits = self.can_claim_spirits;
			<CanClaimSpirits<T>>::put(can_claim_spirits);
			let can_purchase_rare_eggs = self.can_purchase_rare_eggs;
			<CanPurchaseRareEggs<T>>::put(can_purchase_rare_eggs);
			let can_preorder_eggs = self.can_preorder_eggs;
			<CanPreorderEggs<T>>::put(can_preorder_eggs);
		}
	}

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Phala World clock zero day started
		WorldClockStarted {
			start_time: T::BlockNumber,
		},
		/// Start of a new era
		NewEra {
			time: T::BlockNumber,
			era: u64,
		},
		/// Spirit has been claimed from the whitelist
		SpiritClaimed {
			serial_id: SerialId,
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
		/// Spirit Claims status has changed
		ClaimSpiritStatusChanged {
			status: bool,
		},
		/// Purchase Rare Eggs status has changed
		PurchaseRareEggsStatusChanged {
			status: bool,
		},
		/// Preorder Eggs status has changed
		PreorderEggsStatusChanged {
			status: bool,
		},
		OverlordChanged {
			old_overlord: Option<T::AccountId>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		WorldClockAlreadySet,
		AccountNotInWhitelist,
		NoClaimAvailable,
		RareEggPurchaseNotAvailable,
		PreorderEggNotAvailable,
		SpiritAlreadyClaimed,
		ClaimIsOver,
		InsufficientFunds,
		InvalidPurchase,
		InvalidClaimTicket,
		CannotHatchEgg,
		CannotSendFoodToEgg,
		NoFoodAvailable,
		NoPermission,
		CareerAndRaceAlreadyChosen,
		OverlordNotSet,
		RequireOverlordAccount,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T>
		where
		T: pallet_uniques::Config<ClassId = CollectionId, InstanceId = NftId>,
	{
		/// Claim a spirit for users that are on the whitelist. This whitelist will consist of a
		/// a serial id and an account id that is signed by the admin account. When a user comes
		/// to claim their spirit, they will provide a serial id & will be validated as an
		/// authenticated claimer
		///
		/// Parameters:
		/// - origin: The origin of the extrinsic.
		/// - serial_id: The serial id of the spirit to be claimed.
		/// - signature: The signature of the account that is claiming the spirit. //Sr25519Signature
		/// - metadata: The metadata of the account that is claiming the spirit.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		#[transactional]
		pub fn claim_spirit(
			origin: OriginFor<T>,
			serial_id: SerialId,
			signature: BoundedVec<u8, T::StringLimit>, // TODO: change to Signature
			metadata: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			ensure!(CanClaimSpirits::<T>::get(), Error::<T>::ClaimIsOver);
			let sender = ensure_signed(origin.clone())?;
			let overlord = <Overlord<T>>::get();
			match overlord {
				None => Err(Error::<T>::OverlordNotSet.into()),
				Some(overlord) => {
					// Has the SerialId already been claimed
					ensure!(!ClaimedSpirits::<T>::contains_key(serial_id), Error::<T>::SpiritAlreadyClaimed);

					// TODO: Check if valid SerialId to claim a spirit

					// Mint new Spirit and transfer to sender
					pallet_rmrk_core::Pallet::<T>::mint_nft(
						Origin::<T>::Signed(overlord).into(),
						sender.clone(),
						SPIRIT_COLLECTION_ID,
						None,
						None,
						metadata,
					)?;

					ClaimedSpirits::<T>::insert(serial_id, true);
					Self::deposit_event(Event::SpiritClaimed {
						serial_id,
						owner: sender,
					});

					Ok(())
				}
			}
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
			ensure!(CanPurchaseRareEggs::<T>::get(), Error::<T>::RareEggPurchaseNotAvailable);
			let sender = ensure_signed(origin)?;
			// TODO: Ensure sender has a Spirit before purchasing an Egg
			// Do we want to iterate through owned NFTs for a Spirit NFT since this won't be a highly
			// used function?
			let egg_price = match egg_type {
				2 => {
					FOUNDER_EGG_PRICE
				},
				1 => {
					LEGENDARY_EGG_PRICE
				},
				_ => 0,
			};
			ensure!(egg_price != 0, Error::<T>::InvalidPurchase);
			// Transfer the amount for the rare Egg NFT then mint the egg

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
			ensure!(CanPreorderEggs::<T>::get(), Error::<T>::PreorderEggNotAvailable);
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
		) -> DispatchResult {
			// Ensure OverlordOrigin makes call
			T::OverlordOrigin::ensure_origin(origin)?;

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
			// Ensure OverlordOrigin makes call
			T::OverlordOrigin::ensure_origin(origin)?;

			Ok(())
		}

		/// Priveleged function set the Overlord Admin account of Phala World
		///
		/// Parameters:
		/// - origin: Expected to be called by `OverlordOrigin`
		/// - new_overlord: T::AccountId
		#[pallet::weight(0)]
		pub fn set_overlord(
			origin: OriginFor<T>,
			new_overlord: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResultWithPostInfo {
			// This is a public call, so we ensure that the origin is some signed account.
			let sender = ensure_signed(origin)?;
			ensure!(Self::overlord().map_or(false, |k| sender == k), Error::<T>::RequireOverlordAccount);
			let new_overlord = T::Lookup::lookup(new_overlord)?;
			let old_overlord = <Overlord<T>>::get();

			Overlord::<T>::put(&new_overlord);
			Self::deposit_event(Event::OverlordChanged {
				old_overlord,
			});
			// GameOverlord user does not pay a fee
			Ok(Pays::No.into())
		}

		/// Phala World Zero Day is set to begin the tracking of the official time starting at the
		/// current block when `initialize_world_clock` is called by the `Overlord`
		///
		/// Parameters:
		/// `origin`: Expected to be called by `Overlord` admin account
		#[pallet::weight(0)]
		pub fn initialize_world_clock(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			// Ensure Overlord account makes call
			T::OverlordOrigin::ensure_origin(origin)?;
			// Ensure ZeroDay is None as this can only be set once
			ensure!(Self::zero_day() == None, Error::<T>::WorldClockAlreadySet);

			let zero_day = <frame_system::Pallet<T>>::block_number();

			ZeroDay::<T>::put(&zero_day);
			Self::deposit_event(Event::WorldClockStarted {
				start_time: zero_day,
			});

			Ok(Pays::No.into())
		}

		// TODO: Change the following to helper functions and create a set status function to
		// the appropriate function

		/// Set Spirit Claims with the Overlord admin Account to allow users to claim their
		/// Spirits through the `claim_spirits()` function
		///
		/// Parameters:
		/// `origin`: Expected to be called by `Overlord` admin account
		#[pallet::weight(0)]
		pub fn set_claim_spirits_status(
			origin: OriginFor<T>,
			status: bool,
		) -> DispatchResultWithPostInfo {
			// Ensure Overlord account makes call
			let sender = ensure_signed(origin)?;
			ensure!(Self::overlord().map_or(false, |k| sender == k), Error::<T>::RequireOverlordAccount);
			let claim_spirit_status = <CanClaimSpirits<T>>::get();
			<CanClaimSpirits<T>>::put(!claim_spirit_status);

			Self::deposit_event(Event::ClaimSpiritStatusChanged {
				status: !claim_spirit_status,
			});

			Ok(Pays::No.into())
		}

		/// Set Rare Eggs status for purchase with the Overlord Admin Account to allow
		/// users to claim their Spirits through the `claim_spirits()` function
		///
		/// Parameters:
		/// `origin`: Expected to be called by `Overlord` admin account
		#[pallet::weight(0)]
		pub fn set_purchase_rare_eggs_status(
			origin: OriginFor<T>,
			status: bool,
		) -> DispatchResultWithPostInfo {
			// Ensure Overlord account makes call
			let sender = ensure_signed(origin)?;
			ensure!(Self::overlord().map_or(false, |k| sender == k), Error::<T>::RequireOverlordAccount);
			let purchase_rare_eggs_status = <CanPurchaseRareEggs<T>>::get();
			<CanPurchaseRareEggs<T>>::put(!purchase_rare_eggs_status);

			Self::deposit_event(Event::PurchaseRareEggsStatusChanged {
				status: !purchase_rare_eggs_status,
			});

			Ok(Pays::No.into())
		}

		/// Set status of Preordering eggs with the Overlord Admin Account to allow
		/// users to preorder eggs through the `preorder_egg()` function
		///
		/// Parameters:
		/// `origin`: Expected to be called by `Overlord` admin account
		#[pallet::weight(0)]
		pub fn set_preorder_eggs_status(
			origin: OriginFor<T>,
			status: bool,
		) -> DispatchResultWithPostInfo {
			// Ensure Overlord account makes call
			let sender = ensure_signed(origin)?;
			ensure!(Self::overlord().map_or(false, |k| sender == k), Error::<T>::RequireOverlordAccount);
			let preorder_eggs_status = <CanPreorderEggs<T>>::get();
			<CanPreorderEggs<T>>::put(!preorder_eggs_status);

			Self::deposit_event(Event::PreorderEggsStatusChanged {
				status: !preorder_eggs_status,
			});

			Ok(Pays::No.into())
		}
	}
}
