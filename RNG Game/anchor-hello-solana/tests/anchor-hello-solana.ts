/*import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorHelloSolana } from "../target/types/anchor_hello_solana";

describe("anchor-hello-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorHelloSolana as Program<AnchorHelloSolana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});*/
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorHelloSolana } from "../target/types/anchor_hello_solana";

describe("anchor-hello-solana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .anchorHelloSolana as Program<AnchorHelloSolana>;

  it("Says hello!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        user: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
