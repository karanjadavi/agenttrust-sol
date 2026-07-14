use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::AgentTrustError;

#[derive(Accounts)]
pub struct GetCredential<'info> {
    pub credential: Account<'info, Credential>,
}

pub fn handle_get_credential(ctx: Context<GetCredential>) -> Result<()> {
    let credential = &ctx.accounts.credential;

    require!(!credential.revoked, AgentTrustError::AlreadyRevoked);

    msg!("Agent: {}", credential.agent);
    msg!("Issuer: {}", credential.issuer);
    msg!("Credential type: {}", credential.credential_type);
    msg!("Issued at: {}", credential.issued_at);
    msg!("Valid: true");

    Ok(())
}
