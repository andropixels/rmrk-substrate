#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
    pub trait RmrkApi<Block> {
        fn fetch_nft(collection_id: u32, nft_id: u32) -> Option<NftInfo<AccountId, BoundedString>>;
    }
}