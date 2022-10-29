import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { LAMPORTS_PER_SOL } from '@solana/web3.js'
import { expect } from 'chai'
import {
    QUESTS_DATA,
    toAnchorFriendlyQuest,
    toNormalQuest,
} from '../data/quests'
import {
    CRAFT_RECIPES_DATA,
    FORGE_RECIPES_DATA,
    toAnchorFriendlyRecipe,
    toNormalRecipe,
} from '../data/recipes'
import { toAnchorFriendlyBaseStats, toNormalBaseStats } from '../data/stats'
import { Arising } from '../target/types/arising'
import {
    getProgramCharacterAccount,
    getProgramConfigAccount,
    getProgramCraftRecipeAccount,
    getProgramForgeRecipeAccount,
    getProgramQuestAccount,
    getTokenWalletAccount,
} from '../data/accounts'
import { mockMintNFT } from './utils'

describe('arising', () => {
    anchor.setProvider(anchor.AnchorProvider.env())

    const payer = anchor.web3.Keypair.generate()
    const program = anchor.workspace.Arising as Program<Arising>
    const authority = program.provider

    const mint1 = anchor.web3.Keypair.generate()
    const mint2 = anchor.web3.Keypair.generate()
    const mint3 = anchor.web3.Keypair.generate()

    it('Should mint the mock tokens', async () => {
        const airdropSignature = await authority.connection.requestAirdrop(
            payer.publicKey,
            2 * LAMPORTS_PER_SOL
        )
        const latestBlockHash = await authority.connection.getLatestBlockhash()

        await authority.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: airdropSignature,
        })

        await mockMintNFT(payer, anchor.getProvider(), mint1)
        await mockMintNFT(payer, anchor.getProvider(), mint2)
        await mockMintNFT(payer, anchor.getProvider(), mint3)
    })

    it('Initialize', async () => {
        const { account: config_program_address, bump } =
            await getProgramConfigAccount(authority, program)

        await program.methods
            .initialize(bump)
            .accounts({
                config: config_program_address,
                systemProgram: anchor.web3.SystemProgram.programId,
                authority: authority.publicKey,
            })
            .rpc()

        const config = await program.account.config.fetchNullable(
            config_program_address
        )
        expect(config.paused).to.eq(true)
        expect(config.initialized).to.eq(true)
        expect(config.maxCharacters.toNumber()).to.eq(30_000)
        expect(config.secondsBetweenRefreshes.toNumber()).to.eq(86_400)
        expect(config.secondsBetweenPaidRefreshes.toNumber()).to.eq(86_400)
        expect(config.authority.toBase58()).to.eq(
            authority.publicKey.toBase58()
        )
    })

    it('Pause and resume correctly', async () => {
        const { account: config_program_address } =
            await getProgramConfigAccount(authority, program)

        await program.methods
            .setPaused(false)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
            })
            .rpc()

        let config = await program.account.config.fetchNullable(
            config_program_address
        )
        expect(config.paused).to.eq(false)

        await program.methods
            .setPaused(true)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
            })
            .rpc()

        config = await program.account.config.fetchNullable(
            config_program_address
        )
        expect(config.paused).to.eq(true)
    })

    it('Add a fake mint and fetch the information', async () => {
        const { account: config_program_address } =
            await getProgramConfigAccount(authority, program)

        const { account: mint1_address, bump: bump1 } =
            await getProgramCharacterAccount(mint1.publicKey, program)

        await program.methods
            .addCharacter(mint1.publicKey, bump1)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
                character: mint1_address,
            })
            .rpc()

        const { account: mint2_address, bump: bump2 } =
            await getProgramCharacterAccount(mint2.publicKey, program)

        await program.methods
            .addCharacter(mint2.publicKey, bump2)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
                character: mint2_address,
            })
            .rpc()

        const { account: mint3_address, bump: bump3 } =
            await getProgramCharacterAccount(mint3.publicKey, program)

        await program.methods
            .addCharacter(mint3.publicKey, bump3)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
                character: mint3_address,
            })
            .rpc()

        const character1 = await program.account.character.fetchNullable(
            mint1_address
        )
        expect(character1).to.not.be.null
        expect(character1.mint.toString()).to.eq(mint1.publicKey.toString())

        const character2 = await program.account.character.fetchNullable(
            mint2_address
        )
        expect(character2).to.not.be.null
        expect(character2.mint.toString()).to.eq(mint2.publicKey.toString())

        const character3 = await program.account.character.fetchNullable(
            mint3_address
        )
        expect(character3).to.not.be.null
        expect(character3.mint.toString()).to.eq(mint3.publicKey.toString())
    })

    it('Set the initial stats for the initial mints', async () => {
        const assignStats = { might: 2, speed: 2, intellect: 2 }

        const anchorAssignStats = toAnchorFriendlyBaseStats(assignStats)

        const { account: mint1_address } = await getProgramCharacterAccount(
            mint1.publicKey,
            program
        )

        const { account: character1_token_account } =
            await getTokenWalletAccount(authority.publicKey, mint1.publicKey)

        await program.methods
            .assignStatsCharacter(anchorAssignStats)
            .accounts({
                payer: authority.publicKey,
                character: mint1_address,
                characterTokenAccount: character1_token_account,
            })
            .rpc()

        const character1 = await program.account.character.fetchNullable(
            mint1_address
        )
        expect(toNormalBaseStats(character1.baseStats)).to.deep.eq(assignStats)
        expect(toNormalBaseStats(character1.poolStats)).to.deep.eq(assignStats)

        const { account: mint2_address } = await getProgramCharacterAccount(
            mint2.publicKey,
            program
        )

        const { account: character2_token_account } =
            await getTokenWalletAccount(authority.publicKey, mint2.publicKey)

        await program.methods
            .assignStatsCharacter(anchorAssignStats)
            .accounts({
                payer: authority.publicKey,
                character: mint2_address,
                characterTokenAccount: character2_token_account,
            })
            .rpc()

        const character2 = await program.account.character.fetchNullable(
            mint2_address
        )
        expect(toNormalBaseStats(character2.baseStats)).to.deep.eq(assignStats)
        expect(toNormalBaseStats(character2.poolStats)).to.deep.eq(assignStats)

        const { account: mint3_address } = await getProgramCharacterAccount(
            mint3.publicKey,
            program
        )

        const { account: character3_token_account } =
            await getTokenWalletAccount(authority.publicKey, mint3.publicKey)

        await program.methods
            .assignStatsCharacter(anchorAssignStats)
            .accounts({
                payer: authority.publicKey,
                character: mint3_address,
                characterTokenAccount: character3_token_account,
            })
            .rpc()

        const character3 = await program.account.character.fetchNullable(
            mint3_address
        )
        expect(toNormalBaseStats(character3.baseStats)).to.deep.eq(assignStats)
        expect(toNormalBaseStats(character3.poolStats)).to.deep.eq(assignStats)
    })

    it('Add forge recipes', async () => {
        const keys = Object.keys(FORGE_RECIPES_DATA)

        const { account: config_program_address } =
            await getProgramConfigAccount(authority, program)

        for (const key of keys) {
            const recipe = FORGE_RECIPES_DATA[key]

            const { account: recipe_account, bump } =
                await getProgramForgeRecipeAccount(recipe, program)

            const anchorRecipe = toAnchorFriendlyRecipe(recipe)

            await program.methods
                // @ts-ignore
                .addForgeRecipe(bump, new anchor.BN(recipe.id), anchorRecipe)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    forgeRecipe: recipe_account,
                })
                .rpc()
        }
    })

    it('Add craft recipes', async () => {
        const keys = Object.keys(CRAFT_RECIPES_DATA)

        const { account: config_program_address } =
            await getProgramConfigAccount(authority, program)

        for (const key of keys) {
            const recipe = CRAFT_RECIPES_DATA[key]

            const { account: recipe_account, bump } =
                await getProgramCraftRecipeAccount(recipe, program)

            const anchorRecipe = toAnchorFriendlyRecipe(recipe)

            await program.methods
                // @ts-ignore
                .addCraftRecipe(bump, new anchor.BN(recipe.id), anchorRecipe)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    craftRecipe: recipe_account,
                })
                .rpc()
        }
    })

    it('Add quests', async () => {
        const keys = Object.keys(QUESTS_DATA)

        const { account: config_program_address } =
            await getProgramConfigAccount(authority, program)

        for (const key of keys) {
            const quest = QUESTS_DATA[key]

            const { account: quest_account, bump } =
                await getProgramQuestAccount(quest, program)

            const anchorQuest = toAnchorFriendlyQuest(quest)

            await program.methods
                // @ts-ignore
                .addQuest(bump, new anchor.BN(quest.id), anchorQuest)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    quest: quest_account,
                })
                .rpc()
        }
    })

    it('Fetch all forge recipes modify them and compare', async () => {
        const keys = Object.keys(FORGE_RECIPES_DATA)

        const { account: config_program_address } =
            await getProgramConfigAccount(authority, program)

        for (const key of keys) {
            const recipe = FORGE_RECIPES_DATA[key]

            const { account: recipe_account } =
                await getProgramForgeRecipeAccount(recipe, program)

            let anchorRecipe = await program.account.forgeRecipe.fetch(
                recipe_account
            )

            expect(toNormalRecipe(anchorRecipe.recipe)).to.deep.equal(recipe)
            expect(anchorRecipe.recipe.available).to.eq(false)

            await program.methods
                .updateForgeRecipeAvailability(true)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    forgeRecipe: recipe_account,
                })
                .rpc()

            anchorRecipe = await program.account.forgeRecipe.fetch(
                recipe_account
            )

            expect(anchorRecipe.recipe.available).to.eq(true)

            const newName = 'New Recipe Name'
            const anchorNewData = toAnchorFriendlyRecipe(recipe)
            anchorNewData.name = newName

            await program.methods
                // @ts-ignore
                .updateForgeRecipe(anchorNewData)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    forgeRecipe: recipe_account,
                })
                .rpc()

            recipe.name = newName

            anchorRecipe = await program.account.forgeRecipe.fetch(
                recipe_account
            )

            expect(toNormalRecipe(anchorRecipe.recipe)).to.deep.eq(recipe)

            await program.methods
                // @ts-ignore
                .updateForgeRecipe(toAnchorFriendlyRecipe(recipe))
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    forgeRecipe: recipe_account,
                })
                .rpc()
        }
    })

    it('Fetch all craft recipes modify them and compare', async () => {
        const keys = Object.keys(CRAFT_RECIPES_DATA)

        const { account: config_program_address } =
            await getProgramConfigAccount(authority, program)

        for (const key of keys) {
            const recipe = CRAFT_RECIPES_DATA[key]

            const { account: recipe_account } =
                await getProgramCraftRecipeAccount(recipe, program)

            let anchorRecipe = await program.account.craftRecipe.fetch(
                recipe_account
            )

            expect(toNormalRecipe(anchorRecipe.recipe)).to.deep.equal(recipe)
            expect(anchorRecipe.recipe.available).to.eq(false)

            await program.methods
                .updateCraftRecipeAvailability(true)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    craftRecipe: recipe_account,
                })
                .rpc()

            anchorRecipe = await program.account.craftRecipe.fetch(
                recipe_account
            )

            expect(anchorRecipe.recipe.available).to.eq(true)

            const newName = 'New Recipe Name'
            const anchorNewData = toAnchorFriendlyRecipe(recipe)
            anchorNewData.name = newName

            await program.methods
                // @ts-ignore
                .updateCraftRecipe(anchorNewData)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    craftRecipe: recipe_account,
                })
                .rpc()

            recipe.name = newName

            anchorRecipe = await program.account.craftRecipe.fetch(
                recipe_account
            )

            expect(toNormalRecipe(anchorRecipe.recipe)).to.deep.eq(recipe)

            await program.methods
                // @ts-ignore
                .updateCraftRecipe(toAnchorFriendlyRecipe(recipe))
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    craftRecipe: recipe_account,
                })
                .rpc()
        }
    })

    it('Fetch all quests modify them and compare', async () => {
        const keys = Object.keys(QUESTS_DATA)

        const { account: config_program_address } =
            await getProgramConfigAccount(authority, program)

        for (const key of keys) {
            const quest = QUESTS_DATA[key]

            const { account: quest_account } = await getProgramQuestAccount(
                quest,
                program
            )

            let anchorQuest = await program.account.quest.fetch(quest_account)

            expect(toNormalQuest(anchorQuest)).to.deep.equal(quest)
            expect(anchorQuest.available).to.eq(false)

            await program.methods
                .updateQuestAvailability(true)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    quest: quest_account,
                })
                .rpc()

            anchorQuest = await program.account.quest.fetch(quest_account)

            expect(anchorQuest.available).to.eq(true)

            const newName = 'New Quest Name'
            const anchorNewData = toAnchorFriendlyQuest(quest)
            anchorNewData.name = newName

            await program.methods
                // @ts-ignore
                .updateQuest(anchorNewData)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    quest: quest_account,
                })
                .rpc()

            quest.name = newName

            anchorQuest = await program.account.quest.fetch(quest_account)

            expect(toNormalQuest(anchorQuest)).to.deep.eq(quest)

            await program.methods
                // @ts-ignore
                .updateQuest(toAnchorFriendlyQuest(quest))
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    quest: quest_account,
                })
                .rpc()
        }
    })
})
