import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MplCoreAnchorExamples } from "../target/types/mpl_core_anchor_examples";

describe("mpl-core-anchor-examples", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MplCoreAnchorExamples as Program<MplCoreAnchorExamples>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
