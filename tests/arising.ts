import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { expect } from 'chai'
import { Arising } from '../target/types/arising'

const CONFIG_PREFIX = 'arising_config_account'
const CHARACTER_PREFIX = 'arising_character_account'

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
})
