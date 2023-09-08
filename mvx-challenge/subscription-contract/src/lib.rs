#![no_std]

use types::WEGLD_TOKEN_ID;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod admin;
pub mod price;
pub mod private;
pub mod services;
pub mod storage;
pub mod types;

#[multiversx_sc::contract]
pub trait SubscriptionContract:
    storage::StorageModule
    + admin::AdminModule
    + services::ServicesModule
    + price::PriceModule
    + private::PrivateModule
{
    #[init]
    fn init(
        &self,
        enabled: OptionalValue<bool>,
        pair_addresses: OptionalValue<MultiValueEncoded<(ManagedBuffer, ManagedAddress)>>,
    ) {
        match enabled {
            OptionalValue::Some(_val) => self.enabled().set(true),
            OptionalValue::None => {}
        }

        match pair_addresses {
            OptionalValue::Some(val) => self.add_pair_addresses(val),
            OptionalValue::None => {}
        }
    }

    #[payable("*")]
    #[endpoint(deposit)]
    fn deposit(&self) {
        self.require_enabled();

        let (token_id, amount) = self.call_value().egld_or_single_fungible_esdt();
        self.require_token_allowed(&token_id);

        let caller = self.blockchain().get_caller();

        self.balance(&caller, &token_id)
            .update(|val| *val += &amount);
        self.balance_token_ids(&caller).insert(token_id);
    }

    #[endpoint(withdraw)]
    fn withdraw(&self, token_id: EgldOrEsdtTokenIdentifier, amount: BigUint) {
        self.require_enabled();
        self.require_token_allowed(&token_id);

        //maybe check on going subscriptions

        let caller = self.blockchain().get_caller();
        let balance = self.balance(&caller, &token_id).get();
        require!(&balance >= &amount, "not enough balance");

        self.balance(&caller, &token_id)
            .update(|val| *val -= &amount);

        if &balance == &amount {
            self.balance_token_ids(&caller).swap_remove(&token_id);
        }

        self.send().direct(&caller, &token_id, 0u64, &amount);
    }

    #[endpoint(subscribe)]
    fn subscribe(&self, service_provider: ManagedBuffer, services: MultiValueEncoded<usize>) {
        self.require_enabled();

        let caller = self.blockchain().get_caller();
        let now = self.blockchain().get_block_epoch();
        let admin_address = self.admin_service_provider(&service_provider).get();

        for service_number in services.into_iter() {
            //requires
            self.require_service_exists(&service_provider, service_number);
            self.require_user_not_subscribed(&caller, &service_provider, &service_number);

            //find corresponding price
            let (_, corresponding_token) = self.find_price(&service_provider, service_number);

            //if EGLD => token_id = WEGLD //could also just ask users to send wegld to the contract
            if corresponding_token.token_identifier.as_managed_buffer()
                == &ManagedBuffer::from(WEGLD_TOKEN_ID)
            {
                let egld_token_id = EgldOrEsdtTokenIdentifier::egld();
                let amount = corresponding_token.amount;

                //handle egld transfer
                let user_balance =
                    self.handle_egld_transfer(&caller, &egld_token_id, &amount, &admin_address);

                //update user balance and info
                self.update_user_balance_after_subscribing(
                    &caller,
                    &user_balance,
                    &egld_token_id,
                    &amount,
                    &service_provider,
                    now,
                    service_number,
                );
            } else {
                let token_id =
                    EgldOrEsdtTokenIdentifier::esdt(corresponding_token.token_identifier);
                let amount = corresponding_token.amount;

                //handle esdt transfer
                let user_balance =
                    self.handle_esdt_transfer(&caller, &token_id, &amount, &admin_address);

                //update user balance and info
                self.update_user_balance_after_subscribing(
                    &caller,
                    &user_balance,
                    &token_id,
                    &amount,
                    &service_provider,
                    now,
                    service_number,
                );
            }

            //release event
            self.issue_subscribed_to(&caller, &service_provider, service_number);
        }
    }

    #[endpoint(unsubscribe)]
    fn unsubscribe(&self, service_provider: ManagedBuffer, services: MultiValueEncoded<usize>) {
        self.require_enabled();

        let caller = self.blockchain().get_caller();
        for service_number in services.into_iter() {
            //check if user is subscribed
            self.require_user_is_subscribed(&service_provider, service_number, &caller);

            //unsubscribe user
            self.unsubscribe_user_from_service(&service_provider, service_number, &caller);
        }
    }

    #[event("issue-subscribed-to")]
    fn issue_subscribed_to(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] service_provider: &ManagedBuffer,
        #[indexed] service_number: usize,
    );
}
