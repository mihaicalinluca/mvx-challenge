multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait TokenModule: crate::storage::StorageModule {
    //FUNGIBLE TOKENS
    fn issue_fungible_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
        num_decimals: usize,
        issue_cost: BigUint,
        caller: &ManagedAddress,
    ) {
        self.fungible_token(&token_ticker, caller).issue(
            issue_cost,
            token_display_name,
            token_ticker,
            initial_supply,
            num_decimals,
            None,
        );
    }

    fn set_local_roles_fungible(&self, token_ticker: &ManagedBuffer, caller: &ManagedAddress) {
        let roles = [
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftUpdateAttributes,
            EsdtLocalRole::Mint,
            EsdtLocalRole::Burn,
        ];
        self.fungible_token(token_ticker, caller).set_local_roles(&roles, None);
    }

    //needs ESDTLocalRoleMint, used for adding quantity to token
    fn mint_token_fungible_and_send(
        &self,
        amount: BigUint,
        send_to: &ManagedAddress,
        caller: &ManagedAddress,
        token_ticker: &ManagedBuffer
    ) -> EsdtTokenPayment<Self::Api> {
        self.fungible_token(token_ticker, caller).mint_and_send(send_to, amount)
    }

    //needs ESDTLocalRoleBurn
    fn burn_token_fungible(&self, amount: BigUint, token_ticker: &ManagedBuffer, caller: &ManagedAddress) {
        self.fungible_token(token_ticker, caller).burn(&amount)
    }

    //NONFUNGIBLE TOKENS
    fn issue_non_fungible_token(
        &self,
        token_type: EsdtTokenType,
        issue_cost: BigUint,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
        caller: &ManagedAddress,
    ) {
        self.non_fungible_token(&token_ticker, caller).issue(
            token_type,
            issue_cost,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    fn set_roles_non_fungible(&self, token_ticker: &ManagedBuffer, caller: &ManagedAddress) {
        let roles = [
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftAddQuantity,
            EsdtLocalRole::NftBurn,
            EsdtLocalRole::NftUpdateAttributes,
            EsdtLocalRole::Mint,
            EsdtLocalRole::Burn,
        ];
        self.non_fungible_token(token_ticker, caller)
            .set_local_roles(&roles, None);
    }

    //nft
    fn create_nft_and_send(&self, attributes: &ManagedBuffer, token_ticker: &ManagedBuffer, caller: &ManagedAddress) {
        self.non_fungible_token(token_ticker, caller).nft_create_and_send(
            caller,
            BigUint::from(1u64),
            attributes,
        );
    }

    //semi and meta
    fn create_sft_and_send(&self, attributes: &ManagedBuffer, token_ticker: &ManagedBuffer, caller: &ManagedAddress, amount: BigUint) {
        self.non_fungible_token(token_ticker, caller).nft_create_and_send(
            caller,
            amount,
            attributes,
        );
    }

    fn burn_non_fungible(&self, nonce: u64, amount: BigUint, token_ticker: &ManagedBuffer, caller: &ManagedAddress) {
        self.non_fungible_token(token_ticker, caller).nft_burn(nonce, &amount);
    }
}
