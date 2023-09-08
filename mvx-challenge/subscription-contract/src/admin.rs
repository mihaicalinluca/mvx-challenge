multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AdminModule: crate::storage::StorageModule {
    #[only_owner]
    #[endpoint(enableContract)]
    fn enable_contract(&self) {
        self.enabled().set(true)
    }

    #[only_owner]
    #[endpoint(disableContract)]
    fn disable_contract(&self) {
        self.enabled().clear()
    }

    #[only_owner]
    #[endpoint(whitelistTokenIds)]
    fn whitelist_token_ids(&self, token_ids: MultiValueEncoded<EgldOrEsdtTokenIdentifier>) {
        for token_id in token_ids.into_iter() {
            self.whitelisted_token_ids().insert(token_id);
        }
    }

    #[only_owner]
    #[endpoint(removeTokenIds)]
    fn remove_token_ids(&self, token_ids: MultiValueEncoded<EgldOrEsdtTokenIdentifier>) {
        for token_id in token_ids.into_iter() {
            self.whitelisted_token_ids().swap_remove(&token_id);
        }
    }

    #[only_owner]
    #[endpoint(addPairAddresses)]
    fn add_pair_addresses(
        &self,
        pair_addresses: MultiValueEncoded<(ManagedBuffer, ManagedAddress)>,
    ) {
        for (name, pair_address) in pair_addresses.into_iter() {
            self.pair_address(&name).set(pair_address);
        }
    }
}
