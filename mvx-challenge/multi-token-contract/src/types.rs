multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const ISSUE_COST: u64 = 5000000000000000;

#[derive(
    TopEncode,
    TopDecode,
    TypeAbi,
    NestedEncode,
    NestedDecode,
    Clone,
    ManagedVecItem,
    Debug,
    PartialEq,
)]
pub struct Balance<M: ManagedTypeApi> {
    pub user: ManagedAddress<M>,
    pub token_id: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
    pub amount: BigUint<M>,
}

#[derive(
    TopEncode,
    TopDecode,
    TypeAbi,
    NestedEncode,
    NestedDecode,
    Clone,
    ManagedVecItem,
    Debug,
    PartialEq,
)]
pub struct BalanceNoAmount<M: ManagedTypeApi> {
    pub user: ManagedAddress<M>,
    pub token_id: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
}

impl<M: ManagedTypeApi> BalanceNoAmount<M> {
    pub fn into_tuple(self) -> (ManagedAddress<M>, EgldOrEsdtTokenIdentifier<M>, u64) {
        (self.user, self.token_id, self.nonce)
    }
}

#[derive(
    TopEncode,
    TopDecode,
    TypeAbi,
    NestedEncode,
    NestedDecode,
    Clone,
    ManagedVecItem,
    Debug,
    PartialEq,
)]
pub struct Token<M: ManagedTypeApi> {
    pub token_display_name: ManagedBuffer<M>,
    pub token_ticker: ManagedBuffer<M>,
    pub amount: BigUint<M>,
    pub token_type: u8,
    pub attributes: Option<ManagedBuffer<M>>,
    pub num_decimals: Option<usize>,
}

impl<M: ManagedTypeApi> Token<M> {
    pub fn into_tuple(
        self,
    ) -> (
        ManagedBuffer<M>,
        ManagedBuffer<M>,
        BigUint<M>,
        u8,
        OptionalValue<ManagedBuffer<M>>,
        OptionalValue<usize>,
    ) {
        (
            self.token_display_name,
            self.token_ticker,
            self.amount,
            self.token_type,
            OptionalValue::from(self.attributes),
            OptionalValue::from(self.num_decimals),
        )
    }
}

#[derive(
    TopEncode,
    TopDecode,
    TypeAbi,
    NestedEncode,
    NestedDecode,
    Clone,
    ManagedVecItem,
    Debug,
    PartialEq,
)]
pub struct BurnToken<M: ManagedTypeApi> {
    pub nonce: u64,
    pub token_ticker: ManagedBuffer<M>,
    pub amount: BigUint<M>,
    pub token_type: u8,
}

impl<M: ManagedTypeApi> BurnToken<M> {
    pub fn into_tuple(self) -> (u64, ManagedBuffer<M>, BigUint<M>, u8) {
        (self.nonce, self.token_ticker, self.amount, self.token_type)
    }
}
