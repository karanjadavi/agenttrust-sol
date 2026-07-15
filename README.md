# AgentTrust-SOL

On-chain verifiable credential layer for AI agents on Solana, built with Anchor.

## Problem

AI agents on Solana have no standardized, on-chain way to prove their authorization or capability level before interacting with other programs or agents. Every project reinvents ad-hoc trust checks.

## Solution

AgentTrust-SOL lets any issuer create a PDA-based credential account for an agent (keyed by agent pubkey + credential type). Other Solana programs can verify an agent's credential via CPI before trusting it to act.

## Deployment

- Program ID (Devnet): 2My2t9tedGDjfbskGi3p45eZXdvQ76Cxw8AqrXeaN2jp
- Program Explorer: https://explorer.solana.com/address/2My2t9tedGDjfbskGi3p45eZXdvQ76Cxw8AqrXeaN2jp?cluster=devnet

## Live Devnet Transactions (all 3 instructions verified)

- issue_credential: https://explorer.solana.com/tx/2kmFi9wLnfvbZhCj31jKCXXwKg6Rx6q76HQywCTYehhXVFtEpisXuRJkui6J66bGamNW3m5LLXiYnvyxvR45iA4N?cluster=devnet
- get_credential: https://explorer.solana.com/tx/zrNYs1N9E847Qfe2Korg9eYFc96nn9Rh3uFsKXzEN3bsWs4KB3NyVby1wzMoaXKSZcUB4d3Wzbxdjo6sHpFZJdw?cluster=devnet
- revoke_credential: https://explorer.solana.com/tx/47XoZfyqq8zM1mfrqt8DzZTxyrLQ4A2bNg2MpK5sZnKQzS6WNtb5zrLEFi9yBDGXffp93vrCkEDnANN6C9fF2N4T?cluster=devnet

## Instructions

- issue_credential(credential_type) - issuer creates a credential PDA for an agent
- revoke_credential() - issuer revokes a credential they issued
- get_credential() - reads and validates a credential, fails if revoked

## Architecture

Built with Anchor. Credentials are stored as PDAs seeded by [CREDENTIAL_SEED, agent_pubkey, credential_type], making them deterministically derivable and queryable by any program without an off-chain index.

## Testing

Unit tested with litesvm. Run with: cargo test --manifest-path programs/agenttrust/Cargo.toml

End-to-end verified against live Solana devnet via agenttrust/full_flow_devnet.js, exercising all three instructions in sequence.

## Tradeoffs

- Credential type length capped at 32 bytes to bound account size
- Only the original issuer can revoke a credential, no admin override
- No credential expiry currently, revocation is manual

## Status

Complete MVP built for the Superteam Agentic Engineering Grant. All instructions deployed, tested, and verified live on devnet.
