multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod callee_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait CalleeContract {
        #[view(getSafePriceByDefaultOffset)]
        fn get_safe_price_by_default_offset(
            &self,
            pair_address: ManagedAddress, //each pair has a specific address (pool), must be known when calling the endpoint
            input_payment: EsdtTokenPayment,
        ) -> EsdtTokenPayment;
    }
}

#[multiversx_sc::module]
pub trait PriceModule: crate::storage::StorageModule {
    fn get_safe_price(
        &self,
        pair_address: ManagedAddress,
        input_payment: EsdtTokenPayment<Self::Api>,
    ) -> EsdtTokenPayment<Self::Api> {
        let contract_address = self.safe_price_contract_address().get();

        self.contract_proxy(contract_address)
            .get_safe_price_by_default_offset(pair_address, input_payment)
            .async_call() //maybe use transfer_execute?
            .with_callback(self.callbacks().finish_transfer())
            .call_and_exit()
    }

    #[callback]
    fn finish_transfer(
        &self,
        #[call_result] result: ManagedAsyncCallResult<EsdtTokenPayment<Self::Api>>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(_exchanged_token) => {}
            ManagedAsyncCallResult::Err(err) => {
                let error_message = err.err_msg;
                //fail the transaction
                sc_panic!(error_message);
            }
        }
    }

    #[proxy]
    fn contract_proxy(&self, sc_address: ManagedAddress) -> callee_proxy::Proxy<Self::Api>;
}
