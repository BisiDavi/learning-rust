use crate::*;
pub type TokenId = String;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout{
    pub payout: HashMap<AccountId, U128>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata{

}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct TokenMetadata{

}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token{

}

//The Json token is what will be returned from view calls.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken{

}

pub trait NonFungibleTokenMetadata{
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract{
    fn nft_metadata(&self) -> NFTContractMetadata{

    }
}