import { Provider } from '@project-serum/anchor'
import {
    createAssociatedTokenAccountInstruction,
    createMintToInstruction,
    createInitializeMintInstruction,
    MintLayout,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
} from '@solana/spl-token'

import {
    Keypair,
    SystemProgram,
    TransactionInstruction,
    VersionedTransaction,
    TransactionMessage,
} from '@solana/web3.js'

import { getTokenWalletAccount } from '../data/accounts'

// Helper function to mock a NFT mint.
// Creates an associated account with balance of 1 to the provider
export const mockMintNFT = async (
    payer: Keypair,
    provider: Provider,
    mint: Keypair
): Promise<void> => {
    const mintRent =
        await provider.connection.getMinimumBalanceForRentExemption(
            MintLayout.span
        )

    const instructions: TransactionInstruction[] = []

    instructions.push(
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: mint.publicKey,
            lamports: mintRent,
            space: MintLayout.span,
            programId: TOKEN_PROGRAM_ID,
        })
    )

    instructions.push(
        createInitializeMintInstruction(
            mint.publicKey,
            0,
            payer.publicKey,
            payer.publicKey,
            TOKEN_PROGRAM_ID
        )
    )

    const { account: userTokenAccountAddress } = await getTokenWalletAccount(
        provider.publicKey,
        mint.publicKey
    )

    instructions.push(
        createAssociatedTokenAccountInstruction(
            payer.publicKey,
            userTokenAccountAddress,
            provider.publicKey,
            mint.publicKey,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        )
    )

    instructions.push(
        createMintToInstruction(
            mint.publicKey,
            userTokenAccountAddress,
            payer.publicKey,
            1,
            [],
            TOKEN_PROGRAM_ID
        )
    )

    const message = new TransactionMessage({
        payerKey: payer.publicKey,
        recentBlockhash: (await provider.connection.getLatestBlockhash())
            .blockhash,
        instructions,
    }).compileToLegacyMessage()

    const transaction = new VersionedTransaction(message)

    transaction.sign([payer, mint])

    await provider.connection.sendTransaction(transaction)
}

export const waitUntilTimestamp = (timestamp: number): Promise<void> => {
    return new Promise((resolve) => {
        const interval = setInterval(() => {
            checkTime()
        }, 1000)
        const checkTime = () => {
            const now = Math.floor(Date.now() / 1000)
            if (now > timestamp) {
                clearInterval(interval)
                resolve()
            }
        }
    })
}
