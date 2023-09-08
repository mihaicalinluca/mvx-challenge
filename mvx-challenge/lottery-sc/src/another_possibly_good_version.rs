multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::wrong_version::*;

//you can't generate safe random numbers in the same block as the transaction

#[multiversx_sc::module]
pub trait AnotherVersion {
    #[payable("EGLD")]
    #[endpoint(joinLotteryV2)]
    fn join_lottery_v2(&self) {
        let caller = self.blockchain().get_caller();
        let amount = self.call_value().egld_value().clone_value();
        let now_block = self.blockchain().get_block_nonce();

        require!(&amount == &BigUint::from(ONE_EGLD), "invalid payment");

        let mut rand_source = RandomnessSource::new();
        let rand_nr = rand_source.next_u64_in_range(1u64, MAX_NR);
        if rand_nr < 1000 {
            self.is_winner_v2(&caller).set(true); //not sure about this in async calls
        }

        self.joined_block(&caller).set(now_block);
    }

    #[endpoint(claimRewardV2)]
    fn claim_reward_v2(&self) {
        let caller = self.blockchain().get_caller();
        let now_block = self.blockchain().get_block_nonce();

        require!(
            !self.joined_block(&caller).is_empty(),
            "you have to join first"
        );

        require!(
            self.joined_block(&caller).get() != now_block,
            "can't claim in the same block as join"
        );

        require!(
            !self.is_winner_v2(&caller).is_empty(),
            "caller is not winner"
        );

        self.send()
            .direct_egld(&caller, &(BigUint::from(100u64) * ONE_EGLD));

        self.is_winner_v2(&caller).clear();
    }

    #[storage_mapper("isWinnerV2")]
    fn is_winner_v2(&self, user: &ManagedAddress) -> SingleValueMapper<bool>;

    #[storage_mapper("joinedBlock")]
    fn joined_block(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;
}
