import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { AnchorProvider } from "@coral-xyz/anchor";
import { Idl } from "@coral-xyz/anchor";
import { assert } from "chai";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";



describe("counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Counter as Program<Counter>;
  const [counterPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],
    program.programId
  );
  const wallet = provider.wallet;

  before(async () => {
    const balance = await provider.connection.getBalance(wallet.publicKey);
    if (balance < LAMPORTS_PER_SOL) {
      const sig = await provider.connection.requestAirdrop(
        wallet.publicKey,
        LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig);
    }
  });

  it("Initializes the counter to 0", async () => {
    await program.methods
      .initialize()
      .accounts({
        counter: counterPDA,
        user: wallet.publicKey,
        system_program: SystemProgram.programId,
      }as any)
      .rpc();

    const account = await program.account.counter.fetch(counterPDA);
    assert.equal(account.value.toNumber(), 0);
  });

  it("Increments the counter by 1", async () => {
    await program.methods
      .increment()
      .accounts({ counter: counterPDA,
         user: wallet.publicKey }as any)
      .rpc();

    const account = await program.account.counter.fetch(counterPDA);
    assert.equal(account.value.toNumber(), 1);
  });

  it("Increments again to 2", async () => {
    await program.methods
      .increment()
      .accounts({ counter: counterPDA,
         user: wallet.publicKey }as any)
      .rpc();

    const account = await program.account.counter.fetch(counterPDA);
    assert.equal(account.value.toNumber(), 2);
  });

  it("Decrements the counter to 1", async () => {
    await program.methods
      .decrement()
      .accounts({ counter: counterPDA,
         user: wallet.publicKey }as any)
      .rpc();

    const account = await program.account.counter.fetch(counterPDA);
    assert.equal(account.value.toNumber(), 1);
  });

  it("Sets the counter to a custom value (42)", async () => {
    const newValue = new BN(42);
    await program.methods
      .set(newValue)               // ✅ Only the argument here
      .accounts({                  // ✅ Separate `.accounts()` chain
        counter: counterPDA,
        user: wallet.publicKey,
      }as any)
      .rpc();

    const account = await program.account.counter.fetch(counterPDA);
    assert.equal(account.value.toNumber(), 42);
  });

  it("Resets the counter to 0", async () => {
    await program.methods
      .reset()                     // ✅ No arguments
      .accounts({                  // ✅ Always include accounts
        counter: counterPDA,
        user: wallet.publicKey,
      }as any)
      .rpc();

    const account = await program.account.counter.fetch(counterPDA);
    assert.equal(account.value.toNumber(), 0);
  });

  // ✅ FIXED: Check for the actual panic message
  it("Fails when decrementing below 0 (underflow)", async () => {
    await program.methods
      .reset()
      .accounts({ counter: counterPDA, user: wallet.publicKey })
      .rpc();

    try {
      await program.methods
        .decrement()
        .accounts({ counter: counterPDA, user: wallet.publicKey })
        .rpc();
      assert.fail("Expected underflow error but transaction succeeded");
    } catch (error: any) {
      // Program panics with "called `Option::unwrap()` on a `None` value"
      const logs = error.logs || [];
      const logString = logs.join("\n");
      assert.include(
        logString,
        "called `Option::unwrap()` on a `None` value",
        "Expected panic on underflow"
      );
    }
  });

  // ✅ FIXED: Check for the actual panic message
  it("Fails when incrementing beyond u64::MAX (overflow)", async () => {
    const max = new BN("18446744073709551615");
    await program.methods
      .set(max)
      .accounts({ counter: counterPDA, user: wallet.publicKey })
      .rpc();

    try {
      await program.methods
        .increment()
        .accounts({ counter: counterPDA, user: wallet.publicKey })
        .rpc();
      assert.fail("Expected overflow error but transaction succeeded");
    } catch (error: any) {
      // Program panics with "called `Option::unwrap()` on a `None` value"
      const logs = error.logs || [];
      const logString = logs.join("\n");
      assert.include(
        logString,
        "called `Option::unwrap()` on a `None` value",
        "Expected panic on overflow"
      );
    }
  });

});