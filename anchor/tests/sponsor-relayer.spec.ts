import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Basic } from '../target/types/basic';
import { SponsorRelayer } from '../target/types/sponsor_relayer';
import { Keypair, PublicKey, sendAndConfirmTransaction, SYSVAR_INSTRUCTIONS_PUBKEY, Transaction } from '@solana/web3.js';
import { Provider } from 'jotai';
import { AuthorityType, createAccount, createAssociatedTokenAccount, createAssociatedTokenAccountInstruction, createInitializeAccount3Instruction, createMint, createSetAuthorityInstruction, createTransferCheckedInstruction, createWithdrawWithheldTokensFromAccountsInstruction, getAssociatedTokenAddress, mintTo, transferChecked } from '@solana/spl-token';

describe('sponsor-relayer', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SponsorRelayer as Program<SponsorRelayer>;

  const relayer = new Keypair();
  const relayerWallet = new Keypair();
  const sponsor = new Keypair();
  const mint = new Keypair();
  const mint2 = new Keypair();
  const user = new Keypair();
  const user2 = new Keypair();
  const user3 = new Keypair();
  let userTokenAccount: PublicKey;
  let user2TokenAccount: PublicKey;
  let userTokenAccount2: PublicKey;
  let user2TokenAccount2: PublicKey;
  const provider = anchor.AnchorProvider.env();

  it('should setup wallets', async () => {
    
    // airdrop
    const txs = await Promise.all([
      provider.connection.requestAirdrop(relayer.publicKey, 1000000000000),
      provider.connection.requestAirdrop(sponsor.publicKey, 1000000000000),
      provider.connection.requestAirdrop(relayerWallet.publicKey, 1000000000000),
      //provider.connection.requestAirdrop(ta.publicKey, 1000000)
    ]);

    await Promise.all(txs.map(tx => provider.connection.confirmTransaction(tx, "confirmed")));
    // create mint
    await createMint(provider.connection, sponsor, sponsor.publicKey, null, 6, mint);
    await createMint(provider.connection, sponsor, sponsor.publicKey, null, 6, mint2);

    // create token accounts
    userTokenAccount = await createAssociatedTokenAccount(provider.connection, sponsor, mint.publicKey, user.publicKey);
    user2TokenAccount = await createAssociatedTokenAccount(provider.connection, sponsor, mint.publicKey, user2.publicKey);
    userTokenAccount2 = await createAssociatedTokenAccount(provider.connection, sponsor, mint2.publicKey, user.publicKey);
    user2TokenAccount2 = await createAssociatedTokenAccount(provider.connection, sponsor, mint2.publicKey, user2.publicKey);

    // mint tokens
    await mintTo(provider.connection, sponsor, mint.publicKey, userTokenAccount, sponsor, 10000000);
    await mintTo(provider.connection, sponsor, mint2.publicKey, userTokenAccount2, sponsor, 10000000);
    await mintTo(provider.connection, sponsor, mint.publicKey, user2TokenAccount, sponsor, 10000000);
  }, 60000);


  it('should setup sponsorship', async () => {
    // Add your test here.
    
    let tx = await program.methods.initializeSponsorship( new anchor.BN(0),  false,  mint.publicKey, new anchor.BN(1000000000), true).accounts({
      authority: sponsor.publicKey,
    }).signers([sponsor]).rpcAndKeys({skipPreflight: true, commitment: "processed"});

    let sponsorship = tx.pubkeys.sponsorship;

    let tx2 = await program.methods.initializeRelayer().accounts({
      authority: relayer.publicKey,
      sponsorship,
      relayerWallet: relayerWallet.publicKey,
    }).signers([relayer]).rpcAndKeys({skipPreflight: true, commitment: "processed"});

    let relayerAccount = tx2.pubkeys.relayer;

    await program.methods.authorizeRelayer(new anchor.BN(1000000000)).accountsStrict({
      authority: sponsor.publicKey,
      relayer: relayerAccount,
      sponsorship,
    }).signers([sponsor]).rpcAndKeys({skipPreflight: true, commitment: "processed"});


    let transfer = createTransferCheckedInstruction(userTokenAccount, mint.publicKey, user2TokenAccount, user.publicKey, 1000000, 6);

    let relayIx = await program.methods.relay().accountsStrict(
      {
        relayerWallet: relayerWallet.publicKey,
        relayer: relayerAccount,
        sponsorship,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      }
    ).instruction();

    let newTx = new Transaction().add(transfer, relayIx);
    newTx.feePayer = relayerWallet.publicKey;

    await sendAndConfirmTransaction(provider.connection, newTx, [relayerWallet, user], {skipPreflight: true});

    let transfer2 = createTransferCheckedInstruction(userTokenAccount2, mint2.publicKey, user2TokenAccount2, user.publicKey, 1000000, 6);

    let relayIx2 = await program.methods.relay().accountsStrict(
      {
        relayerWallet: relayerWallet.publicKey,
        relayer: relayerAccount,
        sponsorship,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      }
    ).instruction();

    let newTx2 = new Transaction().add(transfer2, relayIx);
    newTx2.feePayer = relayerWallet.publicKey;

    await expect(sendAndConfirmTransaction(provider.connection, newTx2, [relayerWallet, user], {skipPreflight: true})).rejects.toThrow();

    let ata = await getAssociatedTokenAddress(
      mint.publicKey, // mint
      user3.publicKey, // owner
    );

    let taIx = createAssociatedTokenAccountInstruction(relayerWallet.publicKey, ata, user3.publicKey, mint.publicKey);

    let transfer3 = createTransferCheckedInstruction(userTokenAccount, mint.publicKey, ata, user.publicKey, 1000000, 6);

    let relayIx3 = await program.methods.relay().accountsStrict(
      {
        relayerWallet: relayerWallet.publicKey,
        relayer: relayerAccount,
        sponsorship,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      }
    ).remainingAccounts([{pubkey: ata, isWritable: false, isSigner: false}]). instruction();

    let newTx3 = new Transaction().add(taIx, transfer3, relayIx3);
    newTx3.feePayer = relayerWallet.publicKey;

    await expect(sendAndConfirmTransaction(provider.connection, newTx3, [relayerWallet, user], {skipPreflight: true})).rejects.toThrow();


    let ca = createSetAuthorityInstruction(ata, user3.publicKey, AuthorityType.CloseAccount, relayerWallet.publicKey);

    
    let newTx4 = new Transaction().add(taIx, ca, transfer3, relayIx3);
    newTx4.feePayer = relayerWallet.publicKey;

    await sendAndConfirmTransaction(provider.connection, newTx4, [relayerWallet, user, user3], {skipPreflight: true});


  }, 60000);
});
