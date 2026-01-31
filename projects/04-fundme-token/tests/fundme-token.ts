import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FundmeToken } from "../target/types/fundme_token";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync
} from "@solana/spl-token";
import { expect } from "chai";

describe("fundme-token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.FundmeToken as Program<FundmeToken>;
  const admin = (provider.wallet as anchor.Wallet).payer;
  const user = anchor.web3.Keypair.generate();

  let mint: anchor.web3.PublicKey;
  let adminAta: anchor.web3.PublicKey;
  let userAta: anchor.web3.PublicKey;
  let vaultTokenAccount: anchor.web3.PublicKey;

  const [fundDataPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("fund_data")],
    program.programId
  );

  before(async () => {
    // 1. Airdrop SOL to user
    const signature = await provider.connection.requestAirdrop(user.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature);

    // 2. Create the Mock Mint
    mint = await createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      6
    );

    // 3. NOW derive the Vault address (since mint is no longer null)
    vaultTokenAccount = getAssociatedTokenAddressSync(
      mint,
      fundDataPda,
      true // allowOwnerOffCurve = true because fundDataPda is a PDA
    );

    // 4. Create ATAs
    adminAta = (await getOrCreateAssociatedTokenAccount(provider.connection, admin, mint, admin.publicKey)).address;
    userAta = (await getOrCreateAssociatedTokenAccount(provider.connection, admin, mint, user.publicKey)).address;

    // 5. Mint tokens to user
    await mintTo(provider.connection, admin, mint, userAta, admin, 10_000_000);
  });

  it("Initializes the Fund", async () => {
    await program.methods
      .initialize()
      .accounts({
        // @ts-ignore - Some versions of Anchor IDL have trouble with SPL types
        fundData: fundDataPda,
        tokenMint: mint,
        vaultTokenAccount: vaultTokenAccount,
        admin: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    const account = await program.account.fundData.fetch(fundDataPda);
    expect(account.admin.toBase58()).to.equal(admin.publicKey.toBase58());
  });

  it("Allows a user to fund (donate)", async () => {
    const amount = new anchor.BN(1_000_000);

    await program.methods
      .fund(amount)
      .accounts({
        // @ts-ignore
        fundData: fundDataPda,
        user: user.publicKey,
        userTokenAccount: userAta,
        vaultTokenAccount: vaultTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const account = await program.account.fundData.fetch(fundDataPda);
    expect(account.donations.length).to.equal(1);
    expect(account.donations[0].amount.toNumber()).to.equal(amount.toNumber());
  });

  it("Allows admin to withdraw", async () => {
    const withdrawAmount = new anchor.BN(500_000);

    await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        // @ts-ignore
        fundData: fundDataPda,
        vaultTokenAccount: vaultTokenAccount,
        adminTokenAccount: adminAta,
        admin: admin.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const vaultBalance = await provider.connection.getTokenAccountBalance(vaultTokenAccount);
    expect(vaultBalance.value.amount).to.equal("500000"); // Based on previous tests/math
  });
});