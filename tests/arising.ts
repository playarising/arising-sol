import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { expect } from 'chai'
import { Arising } from '../target/types/arising'

const CONFIG_PREFIX = 'arising_config_account'

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

        const configs = await program.account.config.all()
        expect(configs.length).to.eq(1)
        expect(configs[0].account.paused).to.eq(true)
        expect(configs[0].account.initialized).to.eq(true)
        expect(configs[0].account.authority.toBase58()).to.eq(
            authority.publicKey.toBase58()
        )
    })

    it('Pause and resume correctly', async () => {
        const [config_program_address, bump] =
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

        let configs = await program.account.config.all()
        expect(configs[0].account.paused).to.eq(false)

        await program.methods
            .setPaused(true)
            .accounts({
                config: config_program_address,
                payer: authority.publicKey,
            })
            .rpc()

        configs = await program.account.config.all()
        expect(configs[0].account.paused).to.eq(true)
    })
})
