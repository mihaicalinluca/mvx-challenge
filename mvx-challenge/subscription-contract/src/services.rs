multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::*;

#[multiversx_sc::module]
pub trait ServicesModule:
    crate::storage::StorageModule + crate::private::PrivateModule + crate::price::PriceModule
{
    #[endpoint(claimSubscriptionFee)]
    fn claim_subscription_fee(
        &self,
        name_service_provider: ManagedBuffer,
        service_number: usize,
        user: ManagedAddress, //pairs are only USDC -> token
    ) {
        //if user is unsubscribed as a result of this function => next subscription fee should not be claimed
        //send 10 USDC and set pair USDC/EGLD -> price in EGLD
        self.require_enabled();

        let caller = self.blockchain().get_caller();
        let admin_address = self.require_caller_is_service_admin(&caller, &name_service_provider);
        let mut unsubscribed = false;

        let now_epoch = self.blockchain().get_block_epoch();

        //check if user is indeed subscribed to this service
        self.require_user_is_subscribed(&name_service_provider, service_number, &user);

        //check if service has been removed (entry is clear) => unsubscribe user automatically
        unsubscribed = self.check_service_exists(&name_service_provider, service_number, &user);

        //find corresponding price
        let (next_payment, corresponding_token) =
            self.find_price(&name_service_provider, service_number);

        //check if elligible to claim from user (last claim + period)
        self.require_elligible_claim(
            now_epoch,
            next_payment,
            &name_service_provider,
            service_number,
            &user,
        );

        //check if user has balance for claim, if not => unsubscribe
        let amount = corresponding_token.amount;
        if corresponding_token.token_identifier.as_managed_buffer()
            == &ManagedBuffer::from(WEGLD_TOKEN_ID)
        {
            let egld_token_id = EgldOrEsdtTokenIdentifier::egld();
            let user_balance = self.balance(&caller, &egld_token_id).get();
            unsubscribed = self.check_enough_balance(
                &name_service_provider,
                service_number,
                &caller,
                &user_balance,
                &amount,
            );

            //send token to service if user is not unsubscribed by now
            if !unsubscribed {
                //send egld fee to the service
                self.send()
                    .direct(&admin_address, &egld_token_id, 0u64, &amount);

                //update storage
                self.update_user_balance_after_claim(
                    &caller,
                    &user_balance,
                    &egld_token_id,
                    &amount,
                    &name_service_provider,
                    now_epoch,
                    service_number,
                );
            }
        } else {
            let token_id = EgldOrEsdtTokenIdentifier::esdt(corresponding_token.token_identifier);
            let user_balance = self.balance(&caller, &token_id).get();
            unsubscribed = self.check_enough_balance(
                &name_service_provider,
                service_number,
                &caller,
                &user_balance,
                &amount,
            );

            //send token to service if user is not unsubscribed by now
            if !unsubscribed {
                //send esdt fee to the service
                self.send().direct(&admin_address, &token_id, 0u64, &amount);

                //update storage
                self.update_user_balance_after_claim(
                    &caller,
                    &user_balance,
                    &token_id,
                    &amount,
                    &name_service_provider,
                    now_epoch,
                    service_number,
                );
            }
        }
    }

    #[endpoint(registerServiceProvider)]
    fn register_service_provider(
        &self,
        name: ManagedBuffer,
        services: MultiValueEncoded<Service<Self::Api>>,
    ) {
        self.require_enabled();

        require!(
            self.service_providers().insert(name.clone()),
            "service provider already registered"
        );

        let caller = self.blockchain().get_caller();

        for service in services.into_iter() {
            self.require_token_allowed(&service.token_id);

            self.services(&name).push(&service);
        }

        self.admin_service_provider(&name).set(caller);
    }

    #[endpoint(addServices)]
    fn add_services(
        &self,
        name_service_provider: ManagedBuffer,
        services: MultiValueEncoded<Service<Self::Api>>,
    ) {
        self.require_enabled();

        let caller = self.blockchain().get_caller();
        self.require_caller_is_service_admin(&caller, &name_service_provider);

        for service in services.into_iter() {
            self.require_token_allowed(&service.token_id);

            self.services(&name_service_provider).push(&service);
        }
    }

    #[endpoint(replaceServices)]
    fn replace_services(
        &self,
        name_service_provider: ManagedBuffer,
        services: MultiValueEncoded<(usize, Service<Self::Api>)>,
    ) {
        self.require_enabled();

        let caller = self.blockchain().get_caller();

        self.require_caller_is_service_admin(&caller, &name_service_provider);

        for (service_number, service) in services.into_iter() {
            self.services(&name_service_provider)
                .set(service_number, &service);
        }
    }

    #[endpoint(removeServices)]
    fn remove_services(
        &self,
        name_service_provider: ManagedBuffer,
        services: MultiValueEncoded<usize>,
    ) {
        self.require_enabled();

        let caller = self.blockchain().get_caller();

        self.require_caller_is_service_admin(&caller, &name_service_provider);

        for service_number in services.into_iter() {
            self.services(&name_service_provider)
                .clear_entry(service_number);
        }
    }
}
