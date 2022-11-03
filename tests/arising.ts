import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { LAMPORTS_PER_SOL } from '@solana/web3.js'
import { expect } from 'chai'
import { MockFarmQuest, MockJobQuest, MockRaidQuest } from '../data/quests'
import { MockForgeRecipe } from '../data/recipes'
import { Arising } from '../target/types/arising'
import {
    getProgramCharacterAccount,
    getProgramConfigAccount,
    getProgramForgeRecipeAccount,
    getProgramQuestAccount,
    getTokenWalletAccount,
} from '../data/accounts'
import { mockMintNFT, waitUntilTimestamp } from './utils'
import { BASIC_MATERIAL } from '../data/basic_materials'

describe('arising', () => {
    const payer = anchor.web3.Keypair.generate()

    anchor.setProvider(anchor.AnchorProvider.env())

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
            await getProgramConfigAccount(program)

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
        const { account: config_program_address, bump } =
            await getProgramConfigAccount(program)

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
            await getProgramConfigAccount(program)

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

        const { account: mint1_address } = await getProgramCharacterAccount(
            mint1.publicKey,
            program
        )

        const { account: character1_token_account } =
            await getTokenWalletAccount(authority.publicKey, mint1.publicKey)

        await program.methods
            .assignStatsCharacter(assignStats)
            .accounts({
                payer: authority.publicKey,
                character: mint1_address,
                characterTokenAccount: character1_token_account,
            })
            .rpc()

        const character1 = await program.account.character.fetchNullable(
            mint1_address
        )

        expect(character1.baseStats).to.deep.eq(assignStats)
        expect(character1.poolStats).to.deep.eq(assignStats)

        const { account: mint2_address } = await getProgramCharacterAccount(
            mint2.publicKey,
            program
        )

        const { account: character2_token_account } =
            await getTokenWalletAccount(authority.publicKey, mint2.publicKey)

        await program.methods
            .assignStatsCharacter(assignStats)
            .accounts({
                payer: authority.publicKey,
                character: mint2_address,
                characterTokenAccount: character2_token_account,
            })
            .rpc()

        const character2 = await program.account.character.fetchNullable(
            mint2_address
        )
        expect(character2.baseStats).to.deep.eq(assignStats)
        expect(character2.poolStats).to.deep.eq(assignStats)

        const { account: mint3_address } = await getProgramCharacterAccount(
            mint3.publicKey,
            program
        )

        const { account: character3_token_account } =
            await getTokenWalletAccount(authority.publicKey, mint3.publicKey)

        await program.methods
            .assignStatsCharacter(assignStats)
            .accounts({
                payer: authority.publicKey,
                character: mint3_address,
                characterTokenAccount: character3_token_account,
            })
            .rpc()

        const character3 = await program.account.character.fetchNullable(
            mint3_address
        )
        expect(character3.baseStats).to.deep.eq(assignStats)
        expect(character3.poolStats).to.deep.eq(assignStats)
    })

    it('Add mock forge recipe', async () => {
        const { account: config_program_address } =
            await getProgramConfigAccount(program)

        const recipe = MockForgeRecipe()

        const { account: recipe_account, bump } =
            await getProgramForgeRecipeAccount(recipe, program)

        await program.methods
            .addForgeRecipe(bump, recipe.id, recipe)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
                forgeRecipe: recipe_account,
            })
            .rpc()
    })

    it('Add mock quests', async () => {
        const { account: config_program_address } =
            await getProgramConfigAccount(program)

        const quests = [MockJobQuest(), MockFarmQuest(), MockRaidQuest()]

        for (const quest of quests) {
            const { account: quest_account, bump } =
                await getProgramQuestAccount(quest, program)

            await program.methods
                .addQuest(bump, quest.id, quest)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    quest: quest_account,
                })
                .rpc()
        }
    })

    it('Fetch mock forge recipe modify them and compare', async () => {
        const { account: config_program_address } =
            await getProgramConfigAccount(program)

        const recipe = MockForgeRecipe()

        const { account: recipe_account } = await getProgramForgeRecipeAccount(
            recipe,
            program
        )

        let anchorRecipe = await program.account.forgeRecipe.fetch(
            recipe_account
        )

        recipe.available = false

        expect(anchorRecipe.recipe).to.deep.equal(recipe)
        expect(anchorRecipe.recipe.available).to.eq(false)

        await program.methods
            .updateForgeRecipeAvailability(true)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
                forgeRecipe: recipe_account,
            })
            .rpc()

        anchorRecipe = await program.account.forgeRecipe.fetch(recipe_account)

        expect(anchorRecipe.recipe.available).to.eq(true)

        const newName = 'New Recipe Name'
        const newRecipe = recipe
        newRecipe.name = newName

        await program.methods
            .updateForgeRecipe(newRecipe)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
                forgeRecipe: recipe_account,
            })
            .rpc()

        recipe.name = newName

        anchorRecipe = await program.account.forgeRecipe.fetch(recipe_account)

        recipe.available = true

        expect(anchorRecipe.recipe).to.deep.eq(recipe)

        await program.methods
            .updateForgeRecipe(recipe)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
                forgeRecipe: recipe_account,
            })
            .rpc()
    })

    it('Fetch mock quests modify them and compare', async () => {
        const { account: config_program_address } =
            await getProgramConfigAccount(program)

        const quests = [MockJobQuest(), MockFarmQuest(), MockRaidQuest()]

        for (const quest of quests) {
            const { account: quest_account } = await getProgramQuestAccount(
                quest,
                program
            )

            let anchorQuest = await program.account.quest.fetch(quest_account)

            quest.available = false

            expect(anchorQuest).to.deep.equal(quest)
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
            const newQuest = quest
            newQuest.name = newName

            await program.methods
                .updateQuest(newQuest)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    quest: quest_account,
                })
                .rpc()

            quest.name = newName

            anchorQuest = await program.account.quest.fetch(quest_account)

            quest.available = true

            expect(anchorQuest).to.deep.eq(quest)

            await program.methods
                .updateQuest(quest)
                .accounts({
                    config: config_program_address,
                    payer: authority.publicKey,
                    quest: quest_account,
                })
                .rpc()
        }
    })

    it('Start a job quest and claim it', async () => {
        const quest = MockJobQuest()

        const { account: quest_account } = await getProgramQuestAccount(
            quest,
            program
        )

        const { account: character_account } = await getProgramCharacterAccount(
            mint1.publicKey,
            program
        )

        const { account: character_token_account } =
            await getTokenWalletAccount(authority.publicKey, mint1.publicKey)

        await program.methods
            .startQuest(new anchor.BN(Math.random()))
            .accounts({
                character: character_account,
                characterTokenAccount: character_token_account,
                quest: quest_account,
            })
            .rpc()

        let character = await program.account.character.fetch(character_account)

        await waitUntilTimestamp(character.quest.cooldown.toNumber())

        await program.methods
            .claimQuest()
            .accounts({
                character: character_account,
                characterTokenAccount: character_token_account,
                quest: quest_account,
            })
            .rpc()

        character = character = await program.account.character.fetch(
            character_account
        )

        expect(character.raw).to.deep.eq([
            100, 100, 0, 100, 0, 100, 100, 100, 100, 100, 100, 100, 0, 0, 0, 0,
            0,
        ])
    })

    it('Start a farm quest and claim it', async () => {
        const quest = MockFarmQuest()

        const { account: quest_account } = await getProgramQuestAccount(
            quest,
            program
        )

        const { account: character_account } = await getProgramCharacterAccount(
            mint1.publicKey,
            program
        )

        const { account: character_token_account } =
            await getTokenWalletAccount(authority.publicKey, mint1.publicKey)

        await program.methods
            .startQuest(new anchor.BN(Math.random()))
            .accounts({
                character: character_account,
                characterTokenAccount: character_token_account,
                quest: quest_account,
            })
            .rpc()

        let character = await program.account.character.fetch(character_account)

        await waitUntilTimestamp(character.quest.cooldown.toNumber())

        await program.methods
            .claimQuest()
            .accounts({
                character: character_account,
                characterTokenAccount: character_token_account,
                quest: quest_account,
            })
            .rpc()

        character = character = await program.account.character.fetch(
            character_account
        )

        expect(character.raw).to.deep.eq([
            200, 200, 0, 100, 0, 100, 100, 100, 100, 100, 100, 100, 0, 0, 0, 0,
            0,
        ])
    })

    it('Start a raid quest and claim it', async () => {
        const quest = MockRaidQuest()

        const { account: quest_account } = await getProgramQuestAccount(
            quest,
            program
        )

        const { account: character_account } = await getProgramCharacterAccount(
            mint1.publicKey,
            program
        )

        const { account: character_token_account } =
            await getTokenWalletAccount(authority.publicKey, mint1.publicKey)

        await program.methods
            .startQuest(new anchor.BN(Math.random()))
            .accounts({
                character: character_account,
                characterTokenAccount: character_token_account,
                quest: quest_account,
            })
            .rpc()

        let character = await program.account.character.fetch(character_account)

        await waitUntilTimestamp(character.quest.cooldown.toNumber())

        await program.methods
            .claimQuest()
            .accounts({
                character: character_account,
                characterTokenAccount: character_token_account,
                quest: quest_account,
            })
            .rpc()

        character = character = await program.account.character.fetch(
            character_account
        )

        // TODO check level
    })

    it('Refresh the pool points', async () => {
        const { account: config_program_address } =
            await getProgramConfigAccount(program)

        const { account: character_account } = await getProgramCharacterAccount(
            mint1.publicKey,
            program
        )

        const { account: character_token_account } =
            await getTokenWalletAccount(authority.publicKey, mint1.publicKey)

        let character = await program.account.character.fetch(character_account)

        expect(character.poolStats).to.deep.eq({
            might: 1,
            speed: 1,
            intellect: 1,
        })

        await program.methods
            .performRefresh()
            .accounts({
                character: character_account,
                characterTokenAccount: character_token_account,
                config: config_program_address,
            })
            .rpc()

        character = await program.account.character.fetch(character_account)

        expect(character.poolStats).to.deep.eq({
            might: 2,
            speed: 2,
            intellect: 2,
        })
    })

    it('Start a forge recipe and claim it', async () => {
        const recipe = MockForgeRecipe()

        const { account: recipe_account } = await getProgramForgeRecipeAccount(
            recipe,
            program
        )

        const { account: character_account } = await getProgramCharacterAccount(
            mint1.publicKey,
            program
        )

        const { account: character_token_account } =
            await getTokenWalletAccount(authority.publicKey, mint1.publicKey)

        await program.methods
            .startForge()
            .accounts({
                character: character_account,
                characterTokenAccount: character_token_account,
                forgeRecipe: recipe_account,
            })
            .rpc()

        let character = await program.account.character.fetch(character_account)

        await waitUntilTimestamp(character.forge.cooldown.toNumber())

        await program.methods
            .claimForge()
            .accounts({
                character: character_account,
                characterTokenAccount: character_token_account,
                forgeRecipe: recipe_account,
            })
            .rpc()

        character = character = await program.account.character.fetch(
            character_account
        )

        expect(character.basic[BASIC_MATERIAL.WOOD_PLANK - 1]).to.eq(1)
    })
})
