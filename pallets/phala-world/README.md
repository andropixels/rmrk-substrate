# Phala World

## Types
```rust
pub struct WorldClockInfo {
    /// Block number at which world clock started
    zero_day: T::BlockNumber,
    /// Number of eras that have passed
    eras: u128,
}

pub enum EggType {
    /// Egg is a legendary egg
    Legendary,
    /// Egg is a rare egg
    Rare,
    /// Egg is a normal egg
    Normal,
}

pub struct EggTypeInfo<CollectionId, NftId> {
    /// Egg type
    egg_type: EggType,
    /// Hatch timer
    hatch_timer: T::BlockNumber,
    /// Race
    race: Race,
}

```

## Storage
```rust
/// Stores all of the valid claimed spirits from the airdrop
#[pallet::storage]
#[pallet::getter(fn claimed_spirits)]
pub type ClaimedSpirits<T: Config> = StorageMap<_, Twox64Concat, SerialId, bool>;

/// Stores all of the valid claimed Eggs from the whitelist or preorder
#[pallet::storage]
#[pallet::getter(fn claimed_eggs)]
pub type ClaimedEggs<T: Config> = StorageMap<_, Twox64Concat, SerialId, bool>;

/// Preorder info
#[pallet::storage]
#[pallet::getter(fn preorder_info)]
pub type PreOrderInfo<T: Config> = StorageMap<_, Twox64Concat, (SerialId, T::AccountId), (Race, Career)>;

/// Food per Owner
#[pallet::storage]
#[pallet::getter(fn food_by_owner)]
pub type FoodByOwner<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u8>;

/// Eggs hatch time clock
#[pallet::storage]
#[pallet::getter(fn hatch_timer)]
pub type HatchTimer<T: Config> = StorageMap<_, Twox64Concat, (CollectionId, NftId), T::BlockNumber>;

/// Era index for Phala World
#[pallet::storage]
#[pallet::getter(fn era_index)]
pub type EraIndex<T:Config> = StorageValue<_, EraId, ValueQuery>;
```

## Errors
```rust
/// Errors displayed to inform users something went wrong
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
		CareerAndSpeciesAlreadyChosen,
}
```

## Calls
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
		/// Claim a spirit for accounts that are in the whitelist
		#[transactional]
		pub fn claim_spirit(
            origin: OriginFor<T>,
            serial: SerialId,
            signature: Signature,
            metadata: Metadata,
		) -> DisplayResult {
		
		}

		/// Buy a rare egg and choose the Race (limited #). The Duty will be determined
		/// based on activities by account in Phala ecosystem at mint?
		#[transactional]
		pub fn buy_rare_egg(
            origin: OriginFor<T>,
            type: EggType,
            race: Race,
            career: Career,
		) -> DisplayResult {

		}

		/// Preorder will either give the user a claim ticket for an Egg NFT or 
		/// another NFT that will be used to be exchanged for a refund & something 
		/// special TBD 
		#[transactional]
		pub fn preorder_egg(
            origin: OriginFor<T>,
            race: Race,
            career: Career,
		) -> DisplayResult {

		}

		/// Mint Eggs to the new owners
		#[transactional]
		pub fn mint_eggs(
            origin: OriginFor<T>,
            egg_owners: Vec<Serial_Id>,
		) -> DisplayResult {

		}

		/// Claim refund with claim ticket
		#[transactional]
		pub fn claim_refund(
            origin: OriginFor<T>,
            claim_ticket: Serial_Id,
		) -> DisplayResult {

		}

		/// Start hatching egg
		#[transactional]
		pub fn start_hatching( 
            origin: OriginFor<T>,
            collection_id: CollectionId,
            nft_id: NftId,
        ) -> DisplayResult {
            
        }

		/// Send food to an Egg. The owner can be either another egg
		/// egg owner or to the sender's own egg. However, an owner
		/// sending to their own egg can only send twice per era.
		#[transactional]
		pub fn feed_egg(
            origin: OriginFor<T>,
            collection_id: CollectionId,
            nft_id: NftId,
		) -> DisplayResult {

		}

		/// Hatch an egg that is ready for hatching
		pub fn hatch_egg(
            origin: OriginFor<T>,
            collection_id: CollectionId,
            nft_id: NftId,
		) -> DisplayResult {

		}

		/// This will be executed by Phala World for top 10 fed Eggs per era, ensure
		/// origin is Admin Account
		pub fn update_hatch_time(
            origin: OriginFor<T>,
            egg_owner: T::AccountId,
            collection_id: CollectionId,
            nft_id: NftId,
            reduce_time_by: T::BlockNumber
		) -> DisplayResult {

        }
		
}
```
