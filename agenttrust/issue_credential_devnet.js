const anchor = require("@coral-xyz/anchor");
const { PublicKey, Connection, Keypair } = require("@solana/web3.js");
const fs = require("fs");

async function main() {
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  const secretKeyString = fs.readFileSync("/home/hp/.config/solana/id.json", "utf8");
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  const wallet = Keypair.fromSecretKey(secretKey);

  const provider = new anchor.AnchorProvider(
    connection,
    new anchor.Wallet(wallet),
    { commitment: "confirmed" }
  );
  anchor.setProvider(provider);

  const idl = JSON.parse(fs.readFileSync("./target/idl/agenttrust.json", "utf8"));
  const programId = new PublicKey("2My2t9tedGDjfbskGi3p45eZXdvQ76Cxw8AqrXeaN2jp");
  const program = new anchor.Program(idl, provider);

  const agent = Keypair.generate();
  const credentialType = "authorized_api_caller";

  const [credentialPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("credential"),
      agent.publicKey.toBuffer(),
      Buffer.from(credentialType),
    ],
    programId
  );

  console.log("Issuing credential for agent:", agent.publicKey.toBase58());
  console.log("Credential PDA:", credentialPda.toBase58());

  const txSig = await program.methods
    .issueCredential(credentialType)
    .accounts({
      credential: credentialPda,
      agent: agent.publicKey,
      issuer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  console.log("Transaction signature:", txSig);
  console.log("Explorer link:", `https://explorer.solana.com/tx/${txSig}?cluster=devnet`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
