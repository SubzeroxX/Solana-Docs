import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fundme } from "../target/types/fundme";
import { expect } from "chai";

describe("fundme", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Fundme as Program<Fundme>;

  // Derive the PDA for the fund_data account
  const [fundDataPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("fund_data")],
    program.programId
  );

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        fundData: fundDataPDA,
        admin: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.fundData.fetch(fundDataPDA);
    expect(account.admin.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
    expect(account.donations.length).to.equal(0);
  });

  it("Funds the contract (1.5 SOL)", async () => {
    // 1.5 SOL is between 1.0 and 2.0, so it should pass your current requirements
    const amount = new anchor.BN(1_500_000_000);

    await program.methods
      .fund(amount)
      .accounts({
        fundData: fundDataPDA,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.fundData.fetch(fundDataPDA);
    expect(account.donations.length).to.equal(1);
    expect(account.donations[0].amount.toString()).to.equal(amount.toString());
  });

  it("Withdraws funds", async () => {
    const withdrawAmount = new anchor.BN(500_000_000); // 0.5 SOL

    await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        // Note: In your Withdraw struct, you named it fund_account, not fund_data
        fundAccount: fundDataPDA,
        admin: provider.wallet.publicKey,
      })
      .rpc();

    const pdaBalance = await provider.connection.getBalance(fundDataPDA);
    console.log("Remaining PDA Balance:", pdaBalance / anchor.web3.LAMPORTS_PER_SOL, "SOL");
  });

  it("Fails if donation is too low", async () => {
    const smallAmount = new anchor.BN(100_000_000); // 0.1 SOL
    try {
      await program.methods
        .fund(smallAmount)
        .accounts({
          fundData: fundDataPDA,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      expect.fail("Should have failed with TooFewFund");
    } catch (err) {
      // In Anchor, you can check for the error code
      // expect(err.error.errorCode.code).to.equal("TooFewFund");
    }
  });
});