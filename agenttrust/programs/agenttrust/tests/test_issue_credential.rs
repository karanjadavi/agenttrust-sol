use anchor_lang::prelude::*;
use anchor_lang::{InstructionData, ToAccountMetas};
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signer::Signer;
use solana_transaction::Transaction;

#[test]
fn test_issue_and_get_credential() {
    let mut svm = LiteSVM::new();

    let issuer = Keypair::new();
    svm.airdrop(&issuer.pubkey(), 1_000_000_000).unwrap();

    let program_bytes = include_bytes!("../../../target/deploy/agenttrust.so");
    svm.add_program(agenttrust::ID, program_bytes);

    let agent = Keypair::new();
    let credential_type = "authorized_api_caller".to_string();

    let (credential_pda, _bump) = Pubkey::find_program_address(
        &[
            agenttrust::CREDENTIAL_SEED,
            agent.pubkey().as_ref(),
            credential_type.as_bytes(),
        ],
        &agenttrust::ID,
    );

    let accounts = agenttrust::accounts::IssueCredential {
        credential: credential_pda,
        agent: agent.pubkey(),
        issuer: issuer.pubkey(),
        system_program: anchor_lang::system_program::ID,
    };

    let ix_data = agenttrust::instruction::IssueCredential { credential_type: credential_type.clone() }.data();

    let ix = solana_message::Instruction {
        program_id: agenttrust::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data,
    };

    let msg = Message::new(&[ix], Some(&issuer.pubkey()));
    let tx = Transaction::new(&[&issuer], msg, svm.latest_blockhash());

    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "issue_credential failed: {:?}", result.err());

    println!("Credential issued successfully at PDA: {}", credential_pda);
}
