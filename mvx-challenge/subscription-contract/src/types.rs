multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const USDC_TOKEN_ID: &[u8] = b"USDC-c76f1f";
pub const USDC_TICKER: &[u8] = b"USDC";
pub const WEGLD_TOKEN_ID: &[u8] = b"WEGLD-bd4d79";

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
pub struct Service<M: ManagedTypeApi> {
    pub token_id: EgldOrEsdtTokenIdentifier<M>, //token accepted for payment
    pub next_payment: u64, //number of epochs until next payment
    pub price_in_usd: BigUint<M>, //corresponding price in USD
    pub description: ManagedBuffer<M>, //description about the service
}
