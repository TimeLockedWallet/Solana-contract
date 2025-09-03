import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import type { TimeLockedWallet } from "../target/types/time_locked_wallet.ts";
import fs from 'fs';
import path from 'path';
import {
  PublicKey,
  Keypair,
  SystemProgram,
  LAMPORTS_PER_SOL,
  Transaction,
  sendAndConfirmTransaction,
  Connection,
  clusterApiUrl,
} from "@solana/web3.js";
import {
  createMint,
  mintTo,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  createSyncNativeInstruction,
  TOKEN_PROGRAM_ID,
  createTransferInstruction,
  AccountLayout,
} from "@solana/spl-token";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system.js";
import { BN } from "bn.js";
describe("time_locked_wallet", () => {
  let program : Program<TimeLockedWallet>;
  let provider;

  let authority : Wallet;
  let recipient : PublicKey;

  let configPda : PublicKey;
  let vaultPda : PublicKey;
  let bankVaultPda : PublicKey;

  let userUsdcAta : PublicKey;
  let bankVaultUsdcAta : PublicKey;

  const CONFIG_SEED = "CONFIG";
  const VAULT_SEED = "VAULT";
  const BANK_VAULT_SEED = "BANK_VAULT";

  const TOKENS = {
    usdcMint : new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"), 
    wsolMint : new PublicKey("So11111111111111111111111111111111111111112"),
  };

  before(async function() {
    this.timeout(0);

    const idlPath = path.resolve('./target/idl/time_locked_wallet.json');
    const idl = JSON.parse(fs.readFileSync(idlPath, 'utf-8'));

    provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);    
    program = new Program(idl as any, provider)

    authority = provider.wallet as any;
    recipient = provider.wallet as any;

    [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(CONFIG_SEED)],
      program.programId
    );

    [vaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(VAULT_SEED), authority.publicKey.toBuffer()],
      program.programId
    );

    [bankVaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(BANK_VAULT_SEED), authority.publicKey.toBuffer()],
      program.programId
    );

    userUsdcAta = (await getOrCreateAssociatedTokenAccount(
      provider.connection,
      authority.payer,
      TOKENS.usdcMint,
      authority.publicKey,
      true,
    )).address;

    bankVaultUsdcAta = (await getOrCreateAssociatedTokenAccount(
      provider.connection,
      authority.payer,
      TOKENS.usdcMint,
      bankVaultPda,
      true,
    )).address;
  });

  // it("Is initialized!", async () => {
  //   const tx = await program.methods.
  //     initialize(
  //   ).accountsPartial({
  //       config: configPda,
  //       authority: authority.publicKey,
  //       systemProgram: SystemProgram.programId,
  //   }).rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Should successfully initialize the lock!", async () => {
  //   const unlockTimestamp = Math.floor(Date.now() / 1000) + 60;
  //   const amount = new BN(1_000_000); // 1 USDC 
  //   const isSol = true;
  //   const tx = await program.methods.
  //     initializeLock(
  //       new BN(unlockTimestamp),
  //       authority.publicKey,
  //       amount,
  //       isSol
  //   ).accountsPartial({
  //       vault : vaultPda,
  //       bankVault : bankVaultPda,
  //       userTokenAta : userUsdcAta,
  //       bankVaultTokenAta : bankVaultUsdcAta,
  //       user : authority.publicKey,
  //       tokenMint : TOKENS.usdcMint,
  //       associatedTokenProgram : anchor.utils.token.ASSOCIATED_PROGRAM_ID,
  //       systemProgram: SystemProgram.programId,
  //   }).rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Withdraws funds successfully", async () => {
  //   console.log("Current timestamp:", Math.floor(Date.now() / 1000));
  //   const tx = await program.methods.
  //     withdraw(
  //   ).accountsPartial({
  //       vault : vaultPda,
  //       bankVault : bankVaultPda,
  //       bankVaultTokenAta : bankVaultUsdcAta,
  //       recipientTokenAta : userUsdcAta,
  //       owner : authority.publicKey,
  //       tokenMint : TOKENS.usdcMint,
  //       associatedTokenProgram : anchor.utils.token.ASSOCIATED_PROGRAM_ID,
  //       systemProgram: SystemProgram.programId,
  //   }).rpc();
  //   console.log("Your transaction signature", tx);
  // });
});
