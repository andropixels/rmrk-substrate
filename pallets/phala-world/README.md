# Phala World

## Types
```rust
pub enum EggType {
    /// Egg is a normal egg
    Normal,
    /// Egg is a legendary egg
    Legendary,
    /// Egg is a founder egg
    Founder,
}

/// Four Races to choose from
#[derive(Encode, Decode, Clone, PartialEq)]
pub enum RaceType {
    Cyborg,
    AISpectre,
    XGene,
    Pandroid,
}

/// Five Careers to choose from
#[derive(Encode, Decode, Clone, PartialEq)]
pub enum CareerType {
    HardwareDruid,
    RoboWarrior,
    TradeNegotiator,
    HackerWizard,
    Web3Monk,
}
```

### Constants
```rust
/// Seconds per Era that will increment the Era storage value every interval
#[pallet::constant]
type SecondsPerEra: Get<u64>;
/// Price of Founder Egg Price
#[pallet::constant]
type FounderEggPrice: Get<BalanceOf<Self>>;
/// Price of Legendary Egg Price
#[pallet::constant]
type LegendaryEggPrice: Get<BalanceOf<Self>>;
/// Price of Normal Egg Price
#[pallet::constant]
type NormalEggPrice: Get<BalanceOf<Self>>;
/// Max mint per Race
#[pallet::constant]
type MaxMintPerRace: Get<u32>;
/// Max mint per Career
#[pallet::constant]
type MaxMintPerCareer: Get<u32>;
/// Amount of food per Era
#[pallet::constant]
type FoodPerEra: Get<u8>;
/// Max food an Egg can be fed per day
#[pallet::constant]
type MaxFoodFedPerEra: Get<u16>;
/// Max food to feed your own Egg
#[pallet::constant]
type MaxFoodFeedSelf: Get<u8>;
```

## Storage
```rust
/// Stores all of the valid claimed spirits from the airdrop by serial id & bool true if claimed
#[pallet::storage]
#[pallet::getter(fn claimed_spirits)]
pub type ClaimedSpirits<T: Config> = StorageMap<_, Twox64Concat, SerialId, bool>;

/// Stores all of the valid claimed Eggs from the whitelist or preorder
#[pallet::storage]
#[pallet::getter(fn claimed_eggs)]
pub type ClaimedEggs<T: Config> = StorageMap<_, Twox64Concat, SerialId, bool>;

/// Preorder index that is the key to the Preorders StorageMap
#[pallet::storage]
#[pallet::getter(fn preorder_index)]
pub type PreorderIndex<T: Config> = StorageValue<_, PreorderId, ValueQuery>;

/// Preorder info map for user preorders
#[pallet::storage]
#[pallet::getter(fn preorders)]
pub type Preorders<T: Config> = StorageMap<_, Twox64Concat, PreorderId, PreorderInfoOf<T>>;

/// Stores all the Eggs and the information about the Egg pertaining to Hatch times and feeding
#[pallet::storage]
#[pallet::getter(fn eggs)]
pub type Eggs<T: Config> =
StorageDoubleMap<_, Blake2_128Concat, CollectionId, Blake2_128Concat, NftId, EggInfo>;

/// Food per Owner where an owner gets 5 food per era
#[pallet::storage]
#[pallet::getter(fn get_food_by_owner)]
pub type FoodByOwner<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u8>;

/// Phala World Zero Day `BlockNumber` this will be used to determine Eras
#[pallet::storage]
#[pallet::getter(fn zero_day)]
pub(super) type ZeroDay<T: Config> = StorageValue<_, u64>;

/// The current Era from the initial ZeroDay BlockNumber
#[pallet::storage]
#[pallet::getter(fn era)]
pub type Era<T: Config> = StorageValue<_, u64, ValueQuery>;

/// Spirits can be claimed
#[pallet::storage]
#[pallet::getter(fn can_claim_spirits)]
pub type CanClaimSpirits<T: Config> = StorageValue<_, bool, ValueQuery>;

/// Rare Eggs can be purchased
#[pallet::storage]
#[pallet::getter(fn can_purchase_rare_eggs)]
pub type CanPurchaseRareEggs<T: Config> = StorageValue<_, bool, ValueQuery>;

/// Eggs can be preordered
#[pallet::storage]
#[pallet::getter(fn can_preorder_eggs)]
pub type CanPreorderEggs<T: Config> = StorageValue<_, bool, ValueQuery>;

/// Race Type count
#[pallet::storage]
#[pallet::getter(fn race_type_count)]
pub type RaceTypeLeft<T: Config> = StorageMap<_, Twox64Concat, RaceType, u32, ValueQuery>;

/// Race StorageMap count
#[pallet::storage]
#[pallet::getter(fn career_type_count)]
pub type CareerTypeLeft<T: Config> = StorageMap<_, Twox64Concat, CareerType, u32, ValueQuery>;

/// Overlord Admin account of Phala World
#[pallet::storage]
#[pallet::getter(fn overlord)]
pub(super) type Overlord<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

/// Spirit Collection ID
#[pallet::storage]
#[pallet::getter(fn spirit_collection_id)]
pub type SpiritCollectionId<T: Config> = StorageValue<_, CollectionId, OptionQuery>;

/// Egg Collection ID
#[pallet::storage]
#[pallet::getter(fn egg_collection_id)]
pub type EggCollectionId<T: Config> = StorageValue<_, CollectionId, OptionQuery>;
```

## Errors
```rust
/// Errors displayed to inform users something went wrong
#[pallet::error]
pub enum Error<T> {
    WorldClockAlreadySet,
    SpiritClaimNotAvailable,
    RareEggPurchaseNotAvailable,
    PreorderEggNotAvailable,
    SpiritAlreadyClaimed,
    ClaimVerificationFailed,
    InvalidPurchase,
    NoAvailablePreorderId,
    RaceMintMaxReached,
    CareerMintMaxReached,
    CannotHatchEgg,
    CannotSendFoodToEgg,
    NoFoodAvailable,
    OverlordNotSet,
    RequireOverlordAccount,
    InvalidStatusType,
    SpiritCollectionNotSet,
    SpiritCollectionIdAlreadySet,
    EggCollectionNotSet,
    EggCollectionIdAlreadySet,
}
```

## Calls
### claim_spirit
Claim a spirit for users that are on the whitelist.
```rust
origin: OriginFor<T>,
serial_id: SerialId,
signature: sr25519::Signature,
metadata: BoundedVec<u8, T::StringLimit>,
```

### buy_rare_egg
Buy a rare egg of either type Legendary or Founder.
```rust
origin: OriginFor<T>,
egg_type: EggType,
race: RaceType,
career: CareerType,
metadata: BoundedVec<u8, T::StringLimit>,
```

### preorder_egg
Preorder an Egg for eligible users
```rust
origin: OriginFor<T>,
race: RaceType,
career: CareerType,
metadata: BoundedVec<u8, T::StringLimit>,
```

### mint_eggs
This is an admin only function that will be called to do a bulk minting of all preordered egg
```rust
origin: OriginFor<T>
```

### start_hatching
Initiate the hatching phase for an owner's Egg
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
nft_id: NftId,
```

### feed_egg
Feed another egg to the current egg being hatched.
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
nft_id: NftId,
```

### hatch_egg
Hatch the egg that is currently being hatched.
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
nft_id: NftId,
```

### update_hatch_time
This is an admin function to update eggs hatch times based on being in the top 10 of fed eggs within that era
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
nft_id: NftId,
reduced_time: u64,
```

### set_overlord
Privileged function set the Overlord Admin account of Phala World.
```rust
origin: OriginFor<T>,
new_overlord: T::AccountId,
```

### initialize_world_clock
Privileged function where Phala World Zero Day is set to begin the tracking of the official time starting at the current timestamp.
```rust
origin: OriginFor<T>,
```

### set_status_type
Privileged function to set the status for one of the defined StatusTypes like ClaimSpirits, PurchaseRareEggs, or PreorderEggs
```rust
origin: OriginFor<T>,
status: bool,
status_type: StatusType,
```

### set_spirit_collection_id
Privileged function to set the collection id for the Spirits collection
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
```

### set_egg_collection_id
Privileged function to set the collection id for the Egg collection
```rust
origin: OriginFor<T>,
collection_id: CollectionId,
```
