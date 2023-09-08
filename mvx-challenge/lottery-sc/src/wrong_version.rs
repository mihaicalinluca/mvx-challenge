multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const ONE_EGLD: u64 = 1_000_000_000_000_000_000;
pub const MAX_NR: u64 = 100_000;

#[multiversx_sc::module]
pub trait WrongVersion {
    #[payable("EGLD")]
    #[endpoint(wrongEndpoint)]
    fn wrong_endpoint(&self) {
        let payment = self.call_value().egld_value().clone_value();
        require!(payment == BigUint::from(ONE_EGLD), "Invalid payment");

        let mut rand_source = RandomnessSource::new();
        let rand_nr = rand_source.next_u64_in_range(1u64, MAX_NR);
        if rand_nr < 1000 {
            let caller = self.blockchain().get_caller();
            self.send()
                .direct_egld(&caller, &(BigUint::from(100u64) * ONE_EGLD));
        }
    }
}
