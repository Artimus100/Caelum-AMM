import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CaelumAmm } from "../target/types/caelum_amm";
import { NftConversion } from "../target/types/nft_conversion";
import { assert } from "chai";

describe("caelum-amm", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CaelumAmm as Program<CaelumAmm>;
  const nftConversion = anchor.workspace.NftConversion as Program<NftConversion>;

  const payer = anchor.web3.Keypair.generate();
  const bondingCurveAccount = anchor.web3.Keypair.generate();
  const nftConversionState = anchor.web3.Keypair.generate();

  before(async () => {
    const provider = anchor.AnchorProvider.env();
    const signature = await provider.connection.requestAirdrop(payer.publicKey, 1000000000);
    await provider.connection.confirmTransaction(signature);
  });

  it('Initializes the AMM program', async () => {
    const tx = await program.methods.initialize({
      initialPrice: new anchor.BN(1000),
      reserveAmount: new anchor.BN(500)
    })
    .accounts({
      bondingCurve: bondingCurveAccount.publicKey,
      user: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([bondingCurveAccount, payer])
    .rpc();

    console.log('AMM initialized with tx: ', tx);

    // Fetch and verify the bonding curve account
    const accountData = await program.account.bondingCurve.fetch(bondingCurveAccount.publicKey);
    assert.ok(accountData);
  });

  it('Mints tokens in AMM', async () => {
    const tx = await program.methods.mint(new anchor.BN(100))
    .accounts({
      bondingCurve: bondingCurveAccount.publicKey,
      user: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([payer])
    .rpc();

    console.log('Mint transaction: ', tx);

    const accountData = await program.account.bondingCurve.fetch(bondingCurveAccount.publicKey);
    assert.ok(accountData);
  });

  it('Updates AMM parameters', async () => {
    const tx = await program.methods.update({
      newPrice: new anchor.BN(1200),
      newReserve: new anchor.BN(600)
    })
    .accounts({
      bondingCurve: bondingCurveAccount.publicKey,
      admin: payer.publicKey,
    })
    .signers([payer])
    .rpc();

    console.log('Update transaction: ', tx);

    const accountData = await program.account.bondingCurve.fetch(bondingCurveAccount.publicKey);
    assert.ok(accountData);
  });

  it('Creates NFT conversion state and converts NFT', async () => {
    // Create NFT account for testing
    const nftAccount = anchor.web3.Keypair.generate();
    
    const tx = await nftConversion.methods.convert(nftAccount.publicKey)
    .accounts({
      user: payer.publicKey,
      nftAccount: nftAccount.publicKey,
      conversionState: nftConversionState.publicKey,
    })
    .signers([payer])
    .rpc();

    console.log('NFT conversion tx: ', tx);
    assert.ok(tx);
  });

  it('Verifies NFT metadata', async () => {
    const metadata = anchor.web3.Keypair.generate().publicKey;

    const tx = await nftConversion.methods.verify(metadata)
    .accounts({
      user: payer.publicKey,
      nftMetadata: metadata,
      conversionState: nftConversionState.publicKey,
    })
    .signers([payer])
    .rpc();

    console.log('NFT verification tx: ', tx);
    assert.ok(tx);
  });
});