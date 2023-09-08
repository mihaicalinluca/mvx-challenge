#![no_std]

use crate::wrong_version::*;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod another_possibly_good_version;
pub mod wrong_version;

#[multiversx_sc::contract]
pub trait LotterySc:
    wrong_version::WrongVersion + another_possibly_good_version::AnotherVersion
{
    //GOOD VERSION
    #[init]
    fn init(&self) {
        self.is_lottery_open().set(true)
    }

    #[payable("EGLD")]
    #[endpoint(joinLottery)]
    fn join_lottery(&self) {
        let caller = self.blockchain().get_caller();
        let amount = self.call_value().egld_value().clone_value();

        require!(!self.is_lottery_open().is_empty(), "lottery is closed");
        require!(&amount == &BigUint::from(ONE_EGLD), "invalid payment");

        require!(
            self.participants().insert(caller),
            "already joined the lottery"
        );
    }

    #[only_owner]
    #[endpoint(extractWinners)]
    fn extract_winners(&self) {
        let mut rand_source = RandomnessSource::new();

        for user in self.participants().iter() {
            let rand_nr = rand_source.next_u64_in_range(1u64, MAX_NR);
            if rand_nr < 1000 {
                self.is_winner(&user).set(true);
            }
        }

        self.is_lottery_open().clear();
    }

    #[endpoint(claimReward)]
    fn claim_reward(&self) {
        require!(
            self.is_lottery_open().is_empty(),
            "can not claim yet, lottery is still open"
        );

        let caller = self.blockchain().get_caller();

        require!(!self.is_winner(&caller).is_empty(), "caller is not winner");

        self.send()
            .direct_egld(&caller, &(BigUint::from(100u64) * ONE_EGLD));

        self.is_winner(&caller).clear();
    }

    #[view(getIsLotteryOpen)]
    #[storage_mapper("isLotteryOpen")]
    fn is_lottery_open(&self) -> SingleValueMapper<bool>;

    #[view(getAllParticipants)]
    #[storage_mapper("allParticipants")]
    fn participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getIsWinner)]
    #[storage_mapper("isWinner")]
    fn is_winner(&self, user: &ManagedAddress) -> SingleValueMapper<bool>;
}
