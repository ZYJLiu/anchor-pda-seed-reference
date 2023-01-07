import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { PdaSeeds } from "../target/types/pda_seeds"

describe("pda-seeds", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.PdaSeeds as Program<PdaSeeds>

  const PDA = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("seed"),
      program.provider.publicKey.toBuffer(),
      Buffer.from([1]),
      ,
    ],
    program.programId
  )[0]

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(1).accounts({ pda: PDA }).rpc()
    console.log("Your transaction signature", tx)
  })
})
