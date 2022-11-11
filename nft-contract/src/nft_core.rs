pub trait NonFungibleTokenCore{
    //transfers an NFT to a receiver ID
    fn nft_transfer(&mut self, receiver_id:AccountId, token_id:TokenId, memo:Option<String);

    //transfers an NFT to a receiver and calls a function on the receiver ID's contract
    //Returns `true` if the token was transferred from the sender's account.
    fn nft_transfer_call(&mut self, receiver_id:AccountId, token_id:TokenId, memo:Option<String>, msg:String) -> PromiseOrValue<bool>;

    //get information about the NFT token passed in
    fn nft_token(&self, token_id:TokenId) -> Option<JsonToken>;
}

#[ext_contract(ext_non_fungible_token_receiver)]
trait NoNFungibleTokenReceiver{
    //Method stored on the receiver contract that's called via cross contract call when nft_transfer_call is called
    //Returs `true` if the token should be returned back to the sender.

}