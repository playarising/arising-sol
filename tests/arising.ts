import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { expect } from 'chai'
import { toAnchorFriendlyID } from '../data/common'
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
import { Arising } from '../target/types/arising'

const CONFIG_PREFIX = 'arising_config_account'
const CHARACTER_PREFIX = 'arising_character_account'
const FORGE_RECIPE_PREFIX = 'arising_forge_recipe'
const CRAFT_RECIPE_PREFIX = 'arsing_craft'
const QUEST_PREFIX = 'arising_quest'

describe('arising', () => {
    anchor.setProvider(anchor.AnchorProvider.env())

    const program = anchor.workspace.Arising as Program<Arising>
    const authority = program.provider

    it('Initialize', async () => {
        const [config_program_address, bump] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CONFIG_PREFIX), authority.publicKey.toBuffer()],
                program.programId
            )

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
        expect(config.maxCharacters).to.eq(30_000)
        expect(config.secondsBetweenRefreshes).to.eq(86_400)
        expect(config.secondsBetweenPaidRefreshes).to.eq(86_400)
        expect(config.authority.toBase58()).to.eq(
            authority.publicKey.toBase58()
        )
    })

    it('Pause and resume correctly', async () => {
        const [config_program_address] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CONFIG_PREFIX), authority.publicKey.toBuffer()],
                program.programId
            )

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
        const [config_program_address] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CONFIG_PREFIX), authority.publicKey.toBuffer()],
                program.programId
            )

        const mint1 = anchor.web3.Keypair.generate()
        const mint2 = anchor.web3.Keypair.generate()
        const mint3 = anchor.web3.Keypair.generate()

        const [mint_address1, bump1] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CHARACTER_PREFIX), mint1.publicKey.toBuffer()],
                program.programId
            )

        const [mint_address2, bump2] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CHARACTER_PREFIX), mint2.publicKey.toBuffer()],
                program.programId
            )

        const [mint_address3, bump3] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CHARACTER_PREFIX), mint3.publicKey.toBuffer()],
                program.programId
            )

        await program.methods
            .addCharacter(mint1.publicKey, bump1)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
                character: mint_address1,
            })
            .rpc()

        await program.methods
            .addCharacter(mint2.publicKey, bump2)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
                character: mint_address2,
            })
            .rpc()

        await program.methods
            .addCharacter(mint3.publicKey, bump3)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
                character: mint_address3,
            })
            .rpc()

        const character1 = await program.account.character.fetchNullable(
            mint_address1
        )
        expect(character1).to.not.be.null
        expect(character1.mint.toString()).to.eq(mint1.publicKey.toString())

        const character2 = await program.account.character.fetchNullable(
            mint_address2
        )
        expect(character2).to.not.be.null
        expect(character2.mint.toString()).to.eq(mint2.publicKey.toString())

        const character3 = await program.account.character.fetchNullable(
            mint_address3
        )
        expect(character3).to.not.be.null
        expect(character3.mint.toString()).to.eq(mint3.publicKey.toString())
    })

    it('Add forge recipes', async () => {
        const keys = Object.keys(FORGE_RECIPES_DATA)

        const [config_program_address] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CONFIG_PREFIX), authority.publicKey.toBuffer()],
                program.programId
            )

        for (const key of keys) {
            const recipe = FORGE_RECIPES_DATA[key]

            const [recipe_account, bump] =
                await anchor.web3.PublicKey.findProgramAddress(
                    [
                        Buffer.from(FORGE_RECIPE_PREFIX),
                        toAnchorFriendlyID(recipe.id),
                    ],
                    program.programId
                )

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

        const [config_program_address] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CONFIG_PREFIX), authority.publicKey.toBuffer()],
                program.programId
            )

        for (const key of keys) {
            const recipe = CRAFT_RECIPES_DATA[key]

            const [recipe_account, bump] =
                await anchor.web3.PublicKey.findProgramAddress(
                    [
                        Buffer.from(CRAFT_RECIPE_PREFIX),
                        toAnchorFriendlyID(recipe.id),
                    ],
                    program.programId
                )

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

        const [config_program_address] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CONFIG_PREFIX), authority.publicKey.toBuffer()],
                program.programId
            )

        for (const key of keys) {
            const quest = QUESTS_DATA[key]

            const [quest_account, bump] =
                await anchor.web3.PublicKey.findProgramAddress(
                    [Buffer.from(QUEST_PREFIX), toAnchorFriendlyID(quest.id)],
                    program.programId
                )

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

        const [config_program_address] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CONFIG_PREFIX), authority.publicKey.toBuffer()],
                program.programId
            )

        for (const key of keys) {
            const recipe = FORGE_RECIPES_DATA[key]

            const [recipe_account] =
                await anchor.web3.PublicKey.findProgramAddress(
                    [
                        Buffer.from(FORGE_RECIPE_PREFIX),
                        toAnchorFriendlyID(recipe.id),
                    ],
                    program.programId
                )

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

        const [config_program_address] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CONFIG_PREFIX), authority.publicKey.toBuffer()],
                program.programId
            )

        for (const key of keys) {
            const recipe = CRAFT_RECIPES_DATA[key]

            const [recipe_account] =
                await anchor.web3.PublicKey.findProgramAddress(
                    [
                        Buffer.from(CRAFT_RECIPE_PREFIX),
                        toAnchorFriendlyID(recipe.id),
                    ],
                    program.programId
                )

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

        const [config_program_address] =
            await anchor.web3.PublicKey.findProgramAddress(
                [Buffer.from(CONFIG_PREFIX), authority.publicKey.toBuffer()],
                program.programId
            )

        for (const key of keys) {
            const quest = QUESTS_DATA[key]

            const [quest_account] =
                await anchor.web3.PublicKey.findProgramAddress(
                    [Buffer.from(QUEST_PREFIX), toAnchorFriendlyID(quest.id)],
                    program.programId
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

    /*
    it('Fetch all data', async () => {
        const forge_recipes = await program.account.forgeRecipe.all()
        const craft_recipes = await program.account.craftRecipe.all()
        const quests = await program.account.quest.all()
        console.log(forge_recipes, craft_recipes, quests)
    }) */
})
