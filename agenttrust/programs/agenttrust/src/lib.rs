pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2My2t9tedGDjfbskGi3p45eZXdvQ76Cxw8AqrXeaN2jp");

#[program]
pub mod agenttrust {
    use super::*;

    pub fn issue_credential(ctx: Context<IssueCredential>, credential_type: String) -> Result<()> {
        crate::instructions::issue_credential::handle_issue_credential(ctx, credential_type)
    }

    pub fn revoke_credential(ctx: Context<RevokeCredential>) -> Result<()> {
        crate::instructions::revoke_credential::handle_revoke_credential(ctx)
    }

    pub fn get_credential(ctx: Context<GetCredential>) -> Result<()> {
        crate::instructions::get_credential::handle_get_credential(ctx)
    }
}
