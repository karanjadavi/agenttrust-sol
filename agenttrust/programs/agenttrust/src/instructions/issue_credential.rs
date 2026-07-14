use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::*;
use crate::error::AgentTrustError;

#[derive(Accounts)]
#[instruction(credential_type: String)]
pub struct IssueCredential<'info> {
    #[account(
        init,
        payer = issuer,
        space = Credential::MAX_SIZE,
        seeds = [CREDENTIAL_SEED, agent.key().as_ref(), credential_type.as_bytes()],
        bump
    )]
    pub credential: Account<'info, Credential>,

    /// CHECK: agent is just referenced as a pubkey, not read or written directly
    pub agent: UncheckedAccount<'info>,

    #[account(mut)]
    pub issuer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_issue_credential(
    ctx: Context<IssueCredential>,
    credential_type: String,
) -> Result<()> {
    require!(
        credential_type.len() <= MAX_CREDENTIAL_TYPE_LEN,
        AgentTrustError::CredentialTypeTooLong
    );

    let credential = &mut ctx.accounts.credential;
    credential.agent = ctx.accounts.agent.key();
    credential.issuer = ctx.accounts.issuer.key();
    credential.credential_type = credential_type;
    credential.issued_at = Clock::get()?.unix_timestamp;
    credential.revoked = false;
    credential.bump = ctx.bumps.credential;

    Ok(())
}
