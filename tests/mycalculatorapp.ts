const assert = require('assert')
const anchor = require('@project-serum/anchor')
const {SystemProgram} = anchor.web3
describe('mycalculatordapp', () => {
    const provider = anchor.Provider.local();
    anchor.setProvider(provider)
    const calculator = anchor.web3.Keypair.generated()
    const program = anchor.workspace.Mycalculatordapp

})