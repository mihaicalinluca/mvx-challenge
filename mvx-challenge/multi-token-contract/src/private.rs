multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Self::Fungible => 0,
// Self::NonFungible => 1,
// Self::SemiFungible => 2,
// Self::Meta => 3,
// Self::Invalid => 4,

#[multiversx_sc::module]
pub trait PrivateModule:
    crate::storage::StorageModule + crate::token::TokenModule + crate::events::EventsModule
{
    fn check_token_type_and_burn(
        &self,
        token_type: u8,
        token_ticker: &ManagedBuffer,
        caller: &ManagedAddress,
        nonce: u64,
        amount: BigUint,
    ) {
        let mut balance = BigUint::zero();

        match token_type {
            0u8 => {
                //fungible
                let token_id = self.fungible_token(token_ticker, caller).get_token_id();
                balance = self
                    .balance(caller, &EgldOrEsdtTokenIdentifier::esdt(token_id), nonce)
                    .get();
                self.require_enough_balance(&balance, &amount);

                self.burn_token_fungible(amount, token_ticker, caller);
            }
            1_u8..=3u8 => {
                //nft/sft/meta
                let token_id = self.non_fungible_token(token_ticker, caller).get_token_id();
                balance = self
                    .balance(&caller, &EgldOrEsdtTokenIdentifier::esdt(token_id), nonce)
                    .get();

                self.burn_token_fungible(amount, token_ticker, caller)
            }
            4_u8..=u8::MAX => self.require_invalid(),
        }
    }

    fn check_token_type_and_mint(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        amount: BigUint,
        token_type: u8,
        attributes: OptionalValue<ManagedBuffer>,
        num_decimals: OptionalValue<usize>,
        for_user: &ManagedAddress, //for_user
        issue_cost: BigUint,
        _caller: &ManagedAddress,
    ) {
        match token_type {
            4u8 => {
                self.require_invalid();
            }
            1u8 => {
                //is nft
                self.require_nft(
                    &attributes,
                    num_decimals,
                    &EsdtTokenType::NonFungible,
                    &amount,
                );

                //create nft
                self.create_and_mint_nft(
                    &for_user,
                    issue_cost,
                    token_display_name,
                    token_ticker,
                    EsdtTokenType::NonFungible,
                    attributes.into_option().unwrap(),
                );
            }
            0u8 => {
                self.require_fungible(&attributes, &num_decimals);

                //create fungible
                self.create_and_mint_fungible(
                    token_display_name,
                    token_ticker,
                    amount,
                    num_decimals.into_option().unwrap(),
                    issue_cost,
                    &for_user,
                );
            }
            2u8 => {
                //is sft
                self.require_sft(&attributes, &num_decimals);

                //create sft
                self.create_and_mint_sft(
                    &for_user,
                    issue_cost,
                    token_display_name,
                    token_ticker,
                    EsdtTokenType::SemiFungible,
                    attributes.into_option().unwrap(),
                    amount,
                );
            }
            3u8 => {
                //is meta
                self.require_meta(&num_decimals, &attributes);

                //create meta
                self.create_and_mint_meta(
                    &for_user,
                    issue_cost,
                    token_display_name,
                    token_ticker,
                    EsdtTokenType::Meta,
                    attributes.into_option().unwrap(),
                    num_decimals.into_option().unwrap(),
                    amount,
                );
            }
            5_u8..=u8::MAX => {
                self.require_invalid();
            }
        }

        // let from = self.blockchain().get_sc_address();
        // let token_id = TokenIdentifier::from(token_ticker);
        // self.issue_transfer_single(
        //     &from,
        //     &for_user,
        //     &caller,
        //     &EgldOrEsdtTokenIdentifier::esdt(token_id),
        //     &amount,
        // );
    }

    fn create_and_mint_meta(
        &self,
        caller: &ManagedAddress,
        issue_cost: BigUint,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        token_type: EsdtTokenType,
        attributes: ManagedBuffer,
        num_decimals: usize,
        amount: BigUint,
    ) {
        //issue token
        self.issue_non_fungible_token(
            token_type,
            issue_cost,
            token_display_name,
            token_ticker.clone(),
            num_decimals,
            caller,
        );

        //set roles
        self.set_roles_non_fungible(&token_ticker, caller);

        //mint and sent fungible
        self.create_sft_and_send(&attributes, &token_ticker, caller, amount);
    }

    fn create_and_mint_fungible(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
        num_decimals: usize,
        issue_cost: BigUint,
        caller: &ManagedAddress,
    ) {
        //issue token
        self.issue_fungible_token(
            token_display_name,
            token_ticker.clone(),
            initial_supply.clone(),
            num_decimals,
            issue_cost,
            caller,
        );

        //set roles
        self.set_local_roles_fungible(&token_ticker, caller);

        //mint and sent fungible
        self.mint_token_fungible_and_send(initial_supply, caller, caller, &token_ticker);
    }

    fn create_and_mint_nft(
        &self,
        caller: &ManagedAddress,
        issue_cost: BigUint,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        token_type: EsdtTokenType,
        attributes: ManagedBuffer,
    ) {
        //issue token
        self.issue_non_fungible_token(
            token_type,
            issue_cost,
            token_display_name,
            token_ticker.clone(),
            0usize,
            caller,
        );

        //set roles
        self.set_roles_non_fungible(&token_ticker, caller);

        //mint and sent nft
        self.create_nft_and_send(&attributes, &token_ticker, caller);
    }

    fn create_and_mint_sft(
        &self,
        caller: &ManagedAddress,
        issue_cost: BigUint,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        token_type: EsdtTokenType,
        attributes: ManagedBuffer,
        amount: BigUint,
    ) {
        //issue token
        self.issue_non_fungible_token(
            token_type,
            issue_cost,
            token_display_name,
            token_ticker.clone(),
            0usize,
            caller,
        );

        //set roles
        self.set_roles_non_fungible(&token_ticker, caller);

        //mint and sent nft
        self.create_sft_and_send(&attributes, &token_ticker, caller, amount);
    }

    fn require_meta(
        &self,
        num_decimals: &OptionalValue<usize>,
        attributes: &OptionalValue<ManagedBuffer>,
    ) {
        require!(
            num_decimals.is_some() && attributes.is_some(),
            "for meta token num_decimals and attributes should have value"
        );
    }

    fn require_sft(
        &self,
        attributes: &OptionalValue<ManagedBuffer>,
        num_decimals: &OptionalValue<usize>,
    ) {
        require!(
            attributes.is_some() && num_decimals.is_none(),
            "for sft creation attributes should have value, but num_decimals shouldn't"
        );
    }

    fn require_fungible(
        &self,
        attributes: &OptionalValue<ManagedBuffer>,
        num_decimals: &OptionalValue<usize>,
    ) {
        require!(
            num_decimals.is_some(),
            "for fungible token num_decimals hould have value"
        );

        require!(
            attributes.is_none(),
            "for fungible token creation attributes should be none"
        );
    }

    fn require_nft(
        &self,
        attributes: &OptionalValue<ManagedBuffer>,
        num_decimals: OptionalValue<usize>,
        token_type: &EsdtTokenType,
        amount: &BigUint,
    ) {
        require!(
            attributes.is_some()
                && num_decimals.is_none()
                && token_type == &EsdtTokenType::NonFungible,
            "for nft creation attributes should have value, but num_decimals shouldn't"
        );

        require!(amount == &BigUint::from(1u64), "you can only create 1 nft");
    }

    fn require_invalid(&self) {
        sc_panic!("cannot use invalid token type");
    }

    fn require_caller_not_operator(&self, caller: &ManagedAddress, operator: &ManagedAddress) {
        require!(caller != operator, "caller cannot be operator");
    }

    fn require_enabled(&self) {
        require!(!self.enabled().is_empty(), "maintenance")
    }

    fn require_caller_is_approved(&self, caller: &ManagedAddress, user: &ManagedAddress) {
        require!(
            &caller == &user || !self.approved_for_all(&user, &caller).is_empty(),
            "caller not allowed"
        );
    }

    fn require_enough_balance(&self, balance: &BigUint, amount: &BigUint) {
        require!(balance >= amount, "not enough balance");
    }
}
