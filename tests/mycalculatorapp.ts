const assert = require('assert')
const anchor = require('@project-serum/anchor')
const {SystemProgram} = anchor.web3
describe('mycalculatordapp', () => {
    anchor.setProvider(provider)
    const program = anchor.workspace.Mycalculatordapp
    const provider = anchor.getProvider();

    it('Creates a caluclator', async() => {
    })
    const calculator = anchor.web3.Keypair.generate();

    it("Creates a calculator", async () => {
        await program.rpc.create("Welcome to Solana", {
            accounts: {
                calculator: calculator.publicKey,
                user: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [calculator],
        });
        const account = await program.account.calculator.fetch(
            calculator.publicKey
        );
    });
})