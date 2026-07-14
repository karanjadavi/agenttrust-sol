use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::AgentTrustError;

#[derive(Accounts)]
pub struct RevokeCredential<'info> {
    #[account(
        mut,
        has_one = issuer @ AgentTrustError::UnauthorizedRevoke
    )]
    pub credential: Account<'info, Credential>,

    pub issuer: Signer<'info>,
}

pub fn handle_revoke_credential(ctx: Context<RevokeCredential>) -> Result<()> {
    let credential = &mut ctx.accounts.credential;

    require!(!credential.revoked, AgentTrustError::AlreadyRevoked);

    credential.revoked = true;

    Ok(())
}
