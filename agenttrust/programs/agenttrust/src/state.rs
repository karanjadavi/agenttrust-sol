use anchor_lang::prelude::*;

#[account]
pub struct Credential {
    pub agent: Pubkey,
    pub issuer: Pubkey,
    pub credential_type: String,
    pub issued_at: i64,
    pub revoked: bool,
    pub bump: u8,
}

impl Credential {
    pub const MAX_SIZE: usize = 8 + 32 + 32 + (4 + 32) + 8 + 1 + 1;
}
