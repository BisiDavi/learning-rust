use crate::*;
l

#[near_bindgen]
impl Contract {
    //Query for the total supply of NFTs on the Contract
    pub fn nft_total_supply(&self) -> U128{

    }

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn nft_tokens(&self, from_index: Option<U128>, limit:OPtion<u64>) -> Vec<JsonToken>{

    }

    //get the total supply of NFTs for a given owner
    pub fn nft_supply_for_owner(&self, account_id:AccountId) -> U128{

    }

    //Query for all the tokens for an owner
    pub fn nft_tokens_for_owner(&self, account_id:AccountId, from_index:Option<U128>, limit:Option<u64>) -> Vec<JsonToken>{

    }


}