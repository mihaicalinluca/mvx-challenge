multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("issue-transfer-single")]
    fn issue_transfer_single(
        &self,
        #[indexed] from: &ManagedAddress,
        #[indexed] to: &ManagedAddress,
        #[indexed] operator: &ManagedAddress,
        #[indexed] token_id: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    );

    #[event("issue-transfer-batch")]
    fn issue_transfer_batch(
        &self,
        #[indexed] from: &ManagedAddress,
        #[indexed] to: &ManagedAddress,
        #[indexed] operator: &ManagedAddress,
        #[indexed] esdt_transfers: &ManagedVec<EsdtTokenPayment<Self::Api>>,
        #[indexed] egld_amount: &BigUint,
    );

    #[event("issue-approved-for-all")]
    fn issue_approved(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] operator: &ManagedAddress,
        is_approved: bool,
    );
}
