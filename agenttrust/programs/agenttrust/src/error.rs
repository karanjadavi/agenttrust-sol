use anchor_lang::prelude::*;

#[error_code]
pub enum AgentTrustError {
    #[msg("Credential type exceeds maximum length")]
    CredentialTypeTooLong,
    #[msg("Credential has already been revoked")]
    AlreadyRevoked,
    #[msg("Only the issuer can revoke this credential")]
    UnauthorizedRevoke,
}
