import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import {
  ComputeBudgetProgram,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
} from '@solana/web3.js'
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  NATIVE_MINT,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token'

import {
  createAssociatedTokenAccountIfNotExist,
  createMintPair,
  createMarket,
  getAssociatedPoolKeys,
  getMarket,
  sleep,
  createMintToken,
} from './util'

import { PresaleLauncher } from '../target/types/presale_launcher'
import { getAssociatedTokenAddress } from '@solana/spl-token'
import { getOrCreateAssociatedTokenAccount } from '@solana/spl-token'

const globalInfo = {
  marketProgram: new PublicKey('EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj'),
  ammProgram: new PublicKey('HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8'),
  ammCreateFeeDestination: new PublicKey(
    '3XMrhbv989VxAMi3DErLV9eJht1pHppW5LbKxe9fkEFR'
  ),
  market: new Keypair(),
}

const confirmOptions = {
  skipPreflight: false,
}

describe('presale-launcher', () => {
  anchor.setProvider(anchor.AnchorProvider.env())
  const owner = anchor.Wallet.local().payer
  const feePool = Keypair.generate().publicKey
  const program = anchor.workspace.PresaleLauncher as Program<PresaleLauncher>
  const marketId = globalInfo.market.publicKey.toString()
  console.log('market:', marketId.toString())
  it('presale launcher test!', async () => {
    let conn = anchor.getProvider().connection

    let [launcher] = PublicKey.findProgramAddressSync(
      [Buffer.from('launcher')],
      program.programId
    )
    // initialize
    let tx = await program.methods
      .initialize({
        admin: owner.publicKey,
        feePool,
      })
      .accounts({
        signer: owner.publicKey,
        launcher,
        systemProgram: SystemProgram.programId,
      })
      .rpc(confirmOptions)
    console.log('initialize launcher tx:', tx)

    const tokenA = await createMintToken(owner, anchor.getProvider())
    const tokenB = NATIVE_MINT
    const [launchpad] = PublicKey.findProgramAddressSync(
      [Buffer.from('launchpad'), owner.publicKey.toBuffer()],
      program.programId
    )
    const [presaleTreasury] = PublicKey.findProgramAddressSync(
      [Buffer.from('presale_treasury'), launchpad.toBuffer()],
      program.programId
    )
    // open launch pad
    tx = await program.methods
      .initializeLaunchpad({
        owner: owner.publicKey,
        mint: tokenA,
      })
      .accounts({
        signer: owner.publicKey,
        launchpad,
        mint: tokenA,
        systemProgram: SystemProgram.programId,
      })
      .rpc(confirmOptions)
    console.log('initialize launchpad tx:', tx)

    const [presaleTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from('presale'), launchpad.toBuffer()],
      program.programId
    )
    const [reserveTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from('reserve'), launchpad.toBuffer()],
      program.programId
    )
    const sourceTokenAccount = await getOrCreateAssociatedTokenAccount(
      anchor.getProvider().connection,
      owner,
      tokenA,
      owner.publicKey,
      false,
      'processed',
      undefined,
      TOKEN_PROGRAM_ID
    )
    // start presale
    tx = await program.methods
      .startPresale()
      .accounts({
        signer: owner.publicKey,
        launchpad,
        presaleTokenAccount,
        reserveTokenAccount,
        sourceTokenAccount: sourceTokenAccount.address,
        mint: tokenA,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc(confirmOptions)
    console.log('start presale tx:', tx)

    // purchase all token
    tx = await program.methods
      .purchase({
        amount: new anchor.BN('500000000000000000'),
      })
      .accounts({
        signer: owner.publicKey,
        launchpad,
        launcher,
        protoolFeePool: feePool,
        mint: tokenA,
        presaleTreasury,
        presaleTokenAccount,
        userTokenAccount: sourceTokenAccount.address,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc(confirmOptions)
    console.log('purchase tx:', tx)

    // try {
    //   // sell some
    //   tx = await program.methods
    //     .sell({
    //       amount: new anchor.BN('100000000000000000'),
    //     })
    //     .accounts({
    //       signer: owner.publicKey,
    //       launchpad,
    //       launcher,
    //       protoolFeePool: feePool,
    //       mint: tokenA,
    //       presaleTreasury,
    //       presaleTokenAccount,
    //       userTokenAccount: sourceTokenAccount.address,
    //       systemProgram: SystemProgram.programId,
    //       tokenProgram: TOKEN_PROGRAM_ID,
    //     })
    //     .rpc(confirmOptions)
    //   console.log('sell tx:', tx)
    // } catch (error) {
    //   console.log(error)
    // }

    // const { tokenA, tokenB } = await createMintPair(owner, anchor.getProvider())
    // create serum market
    const marketInfo = await createMarket({
      connection: conn,
      wallet: anchor.Wallet.local(),
      baseMint: tokenA,
      quoteMint: tokenB,
      baseLotSize: 1,
      quoteLotSize: 1,
      dexProgram: globalInfo.marketProgram,
      market: globalInfo.market,
    })
    // wait for transaction success
    sleep(60000)

    // get serum market info
    const market = await getMarket(
      conn,
      marketId,
      globalInfo.marketProgram.toString()
    )
    // console.log("market info:", JSON.stringify(market));

    const poolKeys = await getAssociatedPoolKeys({
      programId: globalInfo.ammProgram,
      serumProgramId: globalInfo.marketProgram,
      marketId: market.address,
      baseMint: market.baseMint,
      quoteMint: market.quoteMint,
    })
    // console.log("amm poolKeys: ", JSON.stringify(poolKeys));

    const ammAuthority = poolKeys.authority
    const nonce = poolKeys.nonce
    const ammId: PublicKey = poolKeys.id
    const ammCoinVault: PublicKey = poolKeys.baseVault
    const ammPcVault: PublicKey = poolKeys.quoteVault
    const lpMintAddress: PublicKey = poolKeys.lpMint
    const ammTargetOrders: PublicKey = poolKeys.targetOrders
    const ammOpenOrders: PublicKey = poolKeys.openOrders

    const [amm_config, _] = await getAmmConfigAddress(globalInfo.ammProgram)
    console.log('amm config:', amm_config.toString())
    /************************************ initialize test ***********************************************************************/

    await getOrCreateAssociatedTokenAccount(
      anchor.getProvider().connection,
      owner,
      tokenB,
      owner.publicKey,
      false,
      'processed',
      undefined,
      TOKEN_PROGRAM_ID
    )
    const transaction = new Transaction()
    const userCoinTokenAccount = await createAssociatedTokenAccountIfNotExist(
      owner.publicKey,
      market.baseMint,
      transaction,
      anchor.getProvider().connection
    )

    const userPcTokenAccount = await createAssociatedTokenAccountIfNotExist(
      owner.publicKey,
      market.quoteMint,
      transaction,
      anchor.getProvider().connection
    )
    if (transaction.instructions.length > 0) {
      const txid = anchor.getProvider().send(transaction, null, {
        skipPreflight: true,
        preflightCommitment: 'confirmed',
      })
      console.log('create user lp token account txid:', txid)
    }

    const userLPTokenAccount: PublicKey = await getAssociatedTokenAddress(
      poolKeys.lpMint,
      owner.publicKey
    )

    try {
      tx = await program.methods
        .initializePool(nonce, new anchor.BN(0))
        .accounts({
          launcher,
          protoolFeePool: feePool,
          ammProgram: globalInfo.ammProgram,
          amm: ammId,
          ammAuthority: ammAuthority,
          ammOpenOrders: ammOpenOrders,
          ammLpMint: lpMintAddress,
          ammCoinMint: market.baseMintAddress,
          ammPcMint: market.quoteMintAddress,
          ammCoinVault: ammCoinVault,
          ammPcVault: ammPcVault,
          ammTargetOrders: ammTargetOrders,
          ammConfig: amm_config,
          createFeeDestination: globalInfo.ammCreateFeeDestination,
          marketProgram: globalInfo.marketProgram,
          market: marketId,
          userWallet: owner.publicKey,
          userTokenCoin: userCoinTokenAccount,
          userTokenPc: userPcTokenAccount,
          userTokenLp: userLPTokenAccount,
          presaleTreasury,
          reserveTokenAccount,
          launchpad,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          sysvarRent: SYSVAR_RENT_PUBKEY,
        })
        .preInstructions([
          ComputeBudgetProgram.setComputeUnitLimit({ units: 1400000 }),
        ])
        .rpc(confirmOptions)
      console.log('initialize pool tx: ', tx)
    } catch (error) {
      console.log('initializing pool error:', error)
    }

    console.log(await conn.getTokenAccountBalance(userLPTokenAccount))

    /************************************ withdraw test ***********************************************************************/

    try {
      tx = await program.methods
        .withdraw(
          new anchor.BN(10000000000) // lpAmount
        )
        .accounts({
          ammProgram: globalInfo.ammProgram,
          amm: poolKeys.id,
          ammAuthority: poolKeys.authority,
          ammOpenOrders: poolKeys.openOrders,
          ammTargetOrders: poolKeys.targetOrders,
          ammLpMint: poolKeys.lpMint,
          ammCoinVault: poolKeys.baseVault,
          ammPcVault: poolKeys.quoteVault,
          marketProgram: globalInfo.marketProgram,
          market: marketId,
          marketCoinVault: market.baseVault,
          marketPcVault: market.quoteVault,
          marketVaultSigner: marketInfo.vaultOwner,
          userTokenLp: userLPTokenAccount,
          userTokenCoin: userCoinTokenAccount,
          userTokenPc: userPcTokenAccount,
          userOwner: owner.publicKey,
          marketEventQ: market.eventQueue,
          marketBids: market.bids,
          marketAsks: market.asks,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc(confirmOptions)

      console.log('withdraw tx: ', tx)
    } catch (error) {
      console.log('withdraw error:', error)
    }
  })
})

export async function getAmmConfigAddress(
  programId: PublicKey
): Promise<[PublicKey, number]> {
  const [address, bump] = await PublicKey.findProgramAddress(
    [Buffer.from(anchor.utils.bytes.utf8.encode('amm_config_account_seed'))],
    programId
  )
  return [address, bump]
}
