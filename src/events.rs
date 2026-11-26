use anchor_lang::prelude::*;

#[event]
pub struct VaultInitialized {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub vault_token_account: Pubkey,
}

#[event]
pub struct DepositEvent {
    pub authority: Pubkey,
    pub amount: u64,
    pub total_deposited: u64,
}

#[event]
pub struct WithdrawEvent {
    pub authority: Pubkey,
    pub amount: u64,
    pub total_deposited: u64,
}
