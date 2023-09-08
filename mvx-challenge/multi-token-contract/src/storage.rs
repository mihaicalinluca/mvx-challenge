multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    //ADMIN
    #[storage_mapper("enabled")]
    fn enabled(&self) -> SingleValueMapper<bool>;

    //USERS
    #[storage_mapper("allUsers")]
    fn all_users(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(isApprovedForAll)]
    #[storage_mapper("approvedForAll")]
    fn approved_for_all(
        &self,
        user: &ManagedAddress,
        operator: &ManagedAddress,
    ) -> SingleValueMapper<bool>; //is operator approved for user

    //BALANCE
    #[view(balanceOf)]
    #[storage_mapper("balance")]
    fn balance(
        &self,
        user: &ManagedAddress,
        token_id: &EgldOrEsdtTokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<BigUint>;

    #[storage_mapper("balanceTokenIds")]
    fn balance_token_ids(
        &self,
        user: &ManagedAddress,
    ) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier>;

    //MINT/BURN
    #[storage_mapper("fungibleToken")]
    fn fungible_token(
        &self,
        token_ticker: &ManagedBuffer,
        user: &ManagedAddress,
    ) -> FungibleTokenMapper;

    #[storage_mapper("nonFungibleToken")]
    fn non_fungible_token(
        &self,
        token_ticker: &ManagedBuffer,
        user: &ManagedAddress,
    ) -> NonFungibleTokenMapper;
}
