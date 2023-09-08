multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::*;

#[multiversx_sc::module]
pub trait StorageModule {
    //USER
    #[view(getAllUsers)]
    #[storage_mapper("allUsers")]
    fn all_users(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getSubscribedServices)]
    #[storage_mapper("subscribedServices")]
    fn subscribed_services(
        &self,
        user: &ManagedAddress,
        service_provider: &ManagedBuffer,
    ) -> UnorderedSetMapper<usize>; //service numbers (index)

    #[view(getBalance)]
    #[storage_mapper("balance")]
    fn balance(
        &self,
        user: &ManagedAddress,
        token_id: &EgldOrEsdtTokenIdentifier,
    ) -> SingleValueMapper<BigUint>;

    #[view(getBalanceTokenIds)]
    #[storage_mapper("balanceTokenIds")]
    fn balance_token_ids(
        &self,
        user: &ManagedAddress,
    ) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier>;

    //ADMIN
    #[view(getWhitelistedTokenIds)]
    #[storage_mapper("whitelistedTokenIds")]
    fn whitelisted_token_ids(&self) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getEnabled)]
    #[storage_mapper("enabled")]
    fn enabled(&self) -> SingleValueMapper<bool>;

    //SERVICES
    //service provider => multiple services (addresses)
    #[view(getServiceProviders)]
    #[storage_mapper("serviceProviders")]
    fn service_providers(&self) -> UnorderedSetMapper<ManagedBuffer>;

    #[view(getAdminServiceProvider)]
    #[storage_mapper("adminServiceProvider")]
    fn admin_service_provider(
        &self,
        service_provider: &ManagedBuffer,
    ) -> SingleValueMapper<ManagedAddress>;

    //service can take tokens at a specific time
    #[view(getServices)]
    #[storage_mapper("services")]
    fn services(&self, service_provider: &ManagedBuffer) -> VecMapper<Service<Self::Api>>;

    #[view(getLastClaimedSubscriptionFee)]
    #[storage_mapper("lastClaimedSubscriptionFee")]
    fn last_claimed_subscription_fee(
        &self,
        name_service_provider: &ManagedBuffer,
        service_number: usize,
        user: &ManagedAddress,
    ) -> SingleValueMapper<u64>; //epoch

    //PRICE
    //safePrice contract address
    #[storage_mapper("safePriceContractAddress")]
    fn safe_price_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("pairAddress")]
    fn pair_address(&self, pair_name: &ManagedBuffer) -> SingleValueMapper<ManagedAddress>;
}
