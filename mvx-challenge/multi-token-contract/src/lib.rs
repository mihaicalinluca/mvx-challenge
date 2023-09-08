#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod admin;
pub mod events;
pub mod private;
pub mod storage;
pub mod token;
pub mod types;

use crate::types::*;

#[multiversx_sc::contract]
pub trait MultiTokenContract:
    private::PrivateModule
    + storage::StorageModule
    + admin::AdminModule
    + events::EventsModule
    + token::TokenModule
{
    #[init]
    fn init(&self, enabled: OptionalValue<bool>) {
        match enabled {
            OptionalValue::Some(_val) => self.enabled().set(true),
            OptionalValue::None => {}
        }
    }

    #[payable("*")]
    #[endpoint(deposit)]
    fn deposit(&self) {
        self.require_enabled();

        let token_payment = self.call_value().egld_or_single_esdt();
        let caller = self.blockchain().get_caller();

        let (token_id, nonce, amount) = token_payment.into_tuple();

        self.balance(&caller, &token_id, nonce)
            .update(|val| *val += &amount);

        self.balance_token_ids(&caller).insert(token_id);
    }

    #[endpoint(withdraw)]
    fn withdraw(&self, token: EgldOrEsdtTokenPayment) {
        self.require_enabled();

        let caller = self.blockchain().get_caller();
        let (token_id, nonce, amount) = token.into_tuple();
        let balance = self.balance(&caller, &token_id, nonce).get();

        self.require_enough_balance(&balance, &amount);

        self.balance(&caller, &token_id, nonce)
            .update(|val| *val -= &amount);

        if &balance == &amount {
            self.balance_token_ids(&caller).swap_remove(&token_id);
        }

        self.send().direct(&caller, &token_id, nonce, &amount);
    }

    #[endpoint(balanceOfBatch)]
    fn balance_of_batch(
        &self,
        addresses: MultiValueEncoded<BalanceNoAmount<Self::Api>>,
    ) -> ManagedVec<Balance<Self::Api>> {
        self.require_enabled();

        let mut return_vec = ManagedVec::new();

        for balance_no_amount in addresses.into_iter() {
            let (user, token_id, nonce) = balance_no_amount.into_tuple();

            let balance_per_user = self.balance(&user, &token_id, nonce).get();
            return_vec.push(Balance {
                user,
                token_id,
                nonce,
                amount: balance_per_user,
            });
        }

        return_vec
    }

    #[endpoint(setApprovalForAll)]
    fn set_approval_for_all(&self, operator: ManagedAddress, approved: bool) {
        self.require_enabled();

        let caller = self.blockchain().get_caller();
        self.require_caller_not_operator(&caller, &operator);

        match approved {
            true => self.approved_for_all(&caller, &operator).set(true),
            false => self.approved_for_all(&caller, &operator).clear(),
        }

        //emit event
        self.issue_approved(&caller, &operator, approved);
    }

    #[endpoint(safeTransferFrom)]
    fn safe_transfer_from(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        token: EgldOrEsdtTokenPayment,
    ) {
        self.require_enabled();

        let caller = self.blockchain().get_caller();

        self.require_caller_is_approved(&caller, &from);

        let (token_id, nonce, amount) = token.into_tuple();

        let balance = self.balance(&from, &token_id, nonce).get();
        self.require_enough_balance(&balance, &amount);

        self.send().direct(&to, &token_id, nonce, &amount);

        //also modify balance
        self.balance(&from, &token_id, nonce)
            .update(|val| *val -= &amount);

        //emit event
        self.issue_transfer_single(&from, &to, &caller, &token_id, &amount);
    }

    #[endpoint(safeBatchTransferFrom)]
    fn safe_batch_transfer_from(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        tokens: MultiValueEncoded<EgldOrEsdtTokenPayment>,
    ) {
        self.require_enabled();

        let caller = self.blockchain().get_caller();
        self.require_caller_is_approved(&caller, &from);

        let mut transfer_vec_esdt = ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new();
        let mut transfer_egld = EgldOrEsdtTokenPayment::no_payment();

        for token in tokens.into_iter() {
            let (token_id, nonce, amount) = token.into_tuple();

            let balance = self.balance(&from, &token_id, nonce).get();
            self.require_enough_balance(&balance, &amount);

            if token_id.is_egld() {
                transfer_egld.amount += amount;
            } else {
                //modify balance
                self.balance(&from, &token_id, nonce)
                    .update(|val| *val -= &amount);

                transfer_vec_esdt.push(EsdtTokenPayment::new(
                    token_id.as_esdt_option().unwrap().clone_value(),
                    nonce,
                    amount,
                ));
            }
        }

        if &transfer_egld.amount > &BigUint::zero() {
            self.send().direct_egld(&to, &transfer_egld.amount);

            //modify balance
            self.balance(&from, &transfer_egld.token_identifier, 0u64)
                .update(|val| *val -= &transfer_egld.amount);
        }

        if transfer_vec_esdt.len() != 0usize {
            self.send().direct_multi(&to, &transfer_vec_esdt);
        }

        //emit event
        self.issue_transfer_batch(
            &from,
            &to,
            &caller,
            &transfer_vec_esdt,
            &transfer_egld.amount,
        );
    }

    #[payable("EGLD")]
    #[endpoint(mint)]
    fn mint(
        &self,
        for_user: ManagedAddress,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        amount: BigUint,
        token_type: u8,
        attributes: OptionalValue<ManagedBuffer>,
        num_decimals: OptionalValue<usize>,
    ) {
        self.require_enabled();

        let issue_cost = self.call_value().egld_value().clone_value();

        require!(
            &issue_cost == &BigUint::from(ISSUE_COST),
            "wrong issue cost amount"
        );

        let caller = self.blockchain().get_caller();
        self.require_caller_is_approved(&caller, &for_user);

        self.check_token_type_and_mint(
            token_display_name,
            token_ticker,
            amount,
            token_type,
            attributes,
            num_decimals,
            &for_user,
            issue_cost,
            &caller
        );

    }

    #[payable("EGLD")]
    #[endpoint(mintBatch)]
    fn mint_batch(&self, for_user: ManagedAddress, tokens: MultiValueEncoded<Token<Self::Api>>) {
        self.require_enabled();
        let total_issue_cost = self.call_value().egld_value().clone_value();
        let number_of_tokens = tokens.len();

        require!(
            &total_issue_cost == &(BigUint::from(number_of_tokens) * BigUint::from(ISSUE_COST)),
            "wrong issue cost amount"
        );

        let caller = self.blockchain().get_caller();
        self.require_caller_is_approved(&caller, &for_user);

        for token in tokens.into_iter() {
            let (token_display_name, token_ticker, amount, token_type, attributes, num_decimals) =
                token.into_tuple();

            self.check_token_type_and_mint(
                token_display_name,
                token_ticker,
                amount,
                token_type,
                attributes,
                num_decimals,
                &for_user,
                &total_issue_cost / &BigUint::from(number_of_tokens),
                &caller
            )
        }
    }

    #[endpoint(burn)]
    fn burn(&self, for_user: ManagedAddress, nonce: u64, token_ticker: ManagedBuffer, amount: BigUint, token_type: u8) {
        self.require_enabled();

        let caller = self.blockchain().get_caller();
        self.require_caller_is_approved(&caller, &for_user);

        self.check_token_type_and_burn(token_type, &token_ticker, &for_user, nonce, amount);
    }

    #[endpoint(burnBatch)]
    fn burn_batch(&self, for_user: ManagedAddress, tokens: MultiValueEncoded<BurnToken<Self::Api>>) {
        self.require_enabled();

        let caller = self.blockchain().get_caller();
        self.require_caller_is_approved(&caller, &for_user);

        for token in tokens.into_iter() {
            let (nonce, token_ticker, amount, token_type) = token.into_tuple();
            self.check_token_type_and_burn(token_type, &token_ticker, &for_user, nonce, amount);
        }
    }
}
