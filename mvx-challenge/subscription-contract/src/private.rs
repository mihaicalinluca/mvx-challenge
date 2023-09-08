multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::*;

#[multiversx_sc::module]
pub trait PrivateModule: crate::storage::StorageModule + crate::price::PriceModule {
    fn find_price(
        &self,
        service_provider: &ManagedBuffer,
        service_number: usize,
    ) -> (u64, EsdtTokenPayment<Self::Api>) {
        let service = self.services(service_provider).get(service_number);
        let service_price_in_usd = service.price_in_usd;
        let service_token = service.token_id;

        //get pair address
        let pair_address = self.get_pair_address(&service_token);

        //USDCEGLD pool is actually USDC/WEGLD and returns WEGLD as token id

        //find price
        let corresponding_token = self.get_safe_price(
            pair_address,
            EsdtTokenPayment::new(
                TokenIdentifier::from(USDC_TOKEN_ID),
                0u64,
                service_price_in_usd,
            ),
        ); //result token

        (service.next_payment, corresponding_token)
    }

    fn handle_egld_transfer(
        &self,
        caller: &ManagedAddress,
        egld_token_id: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
        service_address: &ManagedAddress,
    ) -> BigUint {
        //check egld balance instead
        let user_balance = self.balance(caller, egld_token_id).get();
        self.require_enough_balance(&user_balance, amount);

        //take first subscription fee from user 
        //send egld back to the service
        self.send()
            .direct(service_address, egld_token_id, 0u64, amount);

        user_balance
    }

    fn handle_esdt_transfer(
        &self,
        caller: &ManagedAddress,
        token_id: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
        service_address: &ManagedAddress,
    ) -> BigUint {
        //check esdt balance
        let user_balance = self.balance(caller, token_id).get();
        self.require_enough_balance(&user_balance, amount);

        //send esdt back to the service
        self.send().direct(service_address, token_id, 0u64, amount);

        user_balance
    }

    fn get_pair_address(&self, service_token: &EgldOrEsdtTokenIdentifier) -> ManagedAddress {
        let mut pair_name = ManagedBuffer::new_from_bytes(USDC_TICKER);

        //all pairs are USDC -> TOKEN, example would be USDCEGLD
        if service_token.is_egld() {
            pair_name.append_bytes(b"EGLD"); //in reality is USDC/WEGLD pool
        } else {
            let esdt_token = service_token.as_esdt_option().unwrap().clone_value();
            pair_name.append(&esdt_token.ticker());
        }

        require!(
            !self.pair_address(&pair_name).is_empty(),
            "pair address not found for pair"
        );

        self.pair_address(&pair_name).get()
    }

    fn update_user_balance_after_subscribing(
        &self,
        caller: &ManagedAddress,
        user_balance: &BigUint,
        token_id: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
        service_provider: &ManagedBuffer,
        now: u64,
        service_number: usize,
    ) {
        self.balance(caller, token_id).set(user_balance - amount);
        self.last_claimed_subscription_fee(service_provider, service_number, caller)
            .set(now);
        self.subscribed_services(caller, service_provider)
            .insert(service_number);
    }

    fn update_user_balance_after_claim(
        &self,
        caller: &ManagedAddress,
        user_balance: &BigUint,
        token_id: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
        service_provider: &ManagedBuffer,
        now: u64,
        service_number: usize,
    ) {
        self.balance(caller, token_id).set(user_balance - amount);
        self.last_claimed_subscription_fee(service_provider, service_number, caller)
            .set(now);
    }

    fn require_elligible_claim(
        &self,
        now_epoch: u64,
        next_payment: u64,
        name_service_provider: &ManagedBuffer,
        service_number: usize,
        user: &ManagedAddress,
    ) {
        let last_claim = self
            .last_claimed_subscription_fee(&name_service_provider, service_number, &user)
            .get();

        require!(
            last_claim + next_payment <= now_epoch,
            "subscription fee cannot be claimed yet"
        );
    }

    fn require_user_not_subscribed(
        &self,
        caller: &ManagedAddress,
        service_provider: &ManagedBuffer,
        index: &usize,
    ) {
        require!(
            !self
                .subscribed_services(caller, service_provider)
                .contains(index),
            "user already subscribed to one or more services"
        );
    }

    fn require_tokens_are_equal(
        &self,
        first_token: &EgldOrEsdtTokenIdentifier,
        second_token: &EgldOrEsdtTokenIdentifier,
    ) {
        require!(
            first_token == second_token,
            "coresponding token id doesn't match pair address"
        );
    }

    fn require_token_is_egld(&self, token_id: &EgldOrEsdtTokenIdentifier) {
        require!(
            token_id.is_egld(),
            "coresponding token id doesn't match pair address"
        );
    }

    fn require_service_exists(&self, service_provider: &ManagedBuffer, index: usize) {
        require!(
            !self.services(service_provider).item_is_empty(index),
            "service doesn't exist"
        );
    }

    fn check_service_exists(
        &self,
        service_provider: &ManagedBuffer,
        service_number: usize,
        user: &ManagedAddress,
    ) -> bool {
        //service has been removed but user is still subscribed
        if self
            .services(service_provider)
            .item_is_empty(service_number)
        {
            //unsubscribe user
            self.unsubscribe_user_from_service(service_provider, service_number, user);
            return true;
        }

        false
    }

    fn unsubscribe_user_from_service(
        &self,
        service_provider: &ManagedBuffer,
        service_number: usize,
        user: &ManagedAddress,
    ) {
        self.subscribed_services(user, service_provider)
            .swap_remove(&service_number);
    }

    fn require_user_is_subscribed(
        &self,
        service_provider: &ManagedBuffer,
        service_number: usize,
        user: &ManagedAddress,
    ) {
        require!(
            self.subscribed_services(user, service_provider)
                .contains(&service_number),
            "user is not subscribed to one or more services"
        );
    }

    fn require_token_allowed(&self, token_id: &EgldOrEsdtTokenIdentifier) {
        require!(
            self.whitelisted_token_ids().contains(token_id) || token_id.is_egld(),
            "token id not whitelisted"
        );
    }

    fn require_enabled(&self) {
        require!(!self.enabled().is_empty(), "maintenance")
    }

    fn require_caller_is_service_admin(
        &self,
        caller: &ManagedAddress,
        name_service_provider: &ManagedBuffer,
    ) -> ManagedAddress {
        let service_admin = self.admin_service_provider(name_service_provider).get();
        require!(caller == &service_admin, "caller not allowed");

        service_admin
    }

    fn require_enough_balance(&self, user_balance: &BigUint, amount_to_be_paid: &BigUint) {
        require!(
            user_balance >= amount_to_be_paid,
            "not enough balance for subscription fee"
        );
    }

    fn check_enough_balance(
        &self,
        service_provider: &ManagedBuffer,
        service_number: usize,
        user: &ManagedAddress,
        user_balance: &BigUint,
        amount_to_be_paid: &BigUint,
    ) -> bool {
        //user doesn't have enough balance for next subscription fee
        if user_balance < amount_to_be_paid {
            self.unsubscribe_user_from_service(service_provider, service_number, user);
            return true;
        }

        false
    }
}
