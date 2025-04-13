use anchor_lang::prelude::*;


#[event]
pub struct MakeEvent {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub deposit_amt: u64,
    pub receive_amt: u64
}
#[event]
pub struct TakeEvent {
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amt: u64
}
#[event]
pub struct RefundEvent {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
}