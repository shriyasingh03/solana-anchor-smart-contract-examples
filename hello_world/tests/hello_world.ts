import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloWorld } from "../target/types/hello_world";
import { assert } from "chai";

describe("hello_world", () => {
  // Setup the local Solana validator connection
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Load the program from the built IDL
  const program = anchor.workspace.HelloWorld as Program<HelloWorld>;

  // Generate a disposable test wallet
  const user = anchor.web3.Keypair.generate();

  // Airdrop 1 SOL to the test wallet so it can pay fees
  before(async () => {
    const signature = await provider.connection.requestAirdrop(
      user.publicKey,
      1000000000 // 1 SOL
    );
    await provider.connection.confirmTransaction(signature);
    console.log("✅ Airdropped 1 SOL to:", user.publicKey.toString());
  });

  it("Logs 'Hello, Solana!' successfully!", async () => {
    // ---------- BUILD THE TRANSACTION ----------
    const tx = await program.methods
      .sayHello()
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .transaction(); // Build the transaction object

    // Get the latest blockhash (needed for confirmation)
    const blockhash = await provider.connection.getLatestBlockhash();
    tx.feePayer = user.publicKey;
    tx.recentBlockhash = blockhash.blockhash;

    // ---------- SEND THE TRANSACTION ----------
    const txSignature = await provider.connection.sendTransaction(tx, [user]);
    console.log("📝 Transaction Signature:", txSignature);

    // ---------- CONFIRM THE TRANSACTION (WAIT FOR FINALITY) ----------
    const confirmation = await provider.connection.confirmTransaction(
      {
        signature: txSignature,
        blockhash: blockhash.blockhash,
        lastValidBlockHeight: blockhash.lastValidBlockHeight,
      },
      "confirmed" // Wait for "confirmed" commitment
    );

    // Check if the transaction failed
    if (confirmation.value.err) {
      throw new Error(`Transaction failed: ${confirmation.value.err}`);
    }

    console.log("⏳ Transaction confirmed, fetching logs...");

    // ---------- FETCH THE TRANSACTION WITH LOGS ----------
    // Wait an extra 500ms to ensure the RPC has indexed the logs
    await new Promise((resolve) => setTimeout(resolve, 500));

    const transaction = await provider.connection.getTransaction(txSignature, {
      commitment: "confirmed",
    });

    // Safety check: ensure we actually got the transaction data
    if (!transaction) {
      throw new Error("Transaction not found after confirmation.");
    }

    // Extract the logs from the metadata
    const logs = transaction.meta?.logMessages || [];

    // ---------- VERIFY THE LOGS ----------
    // Print the logs to the terminal for debugging (optional but helpful)
    console.log("📜 Raw logs:", logs);

    // Assert that our "Hello, Solana!" message exists
    const helloLog = logs.find((log) => log.includes("Hello, Solana!"));
    assert.exists(helloLog, 'Log should contain "Hello, Solana!"');

    // Assert that the "Called by:" message exists
    const calledByLog = logs.find((log) => log.includes("Called by:"));
    assert.exists(calledByLog, 'Log should contain "Called by:"');

    console.log("✅ All logs verified successfully!");
  });
});