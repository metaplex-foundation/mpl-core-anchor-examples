import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MplCoreAnchorWrapper } from "../target/types/mpl_core_anchor_wrapper";

describe("mpl-core-anchor-examples", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .MplCoreAnchorWrapper as Program<MplCoreAnchorWrapper>;

  it("Can create an Asset", async () => {
    const asset = anchor.web3.Keypair.generate();
    // Add your test here.
    const tx = await program.methods
      .createV1({
        name: "Hello Anchor!",
        uri: "www.example.com",
        plugins: null,
      })
      .accounts({
        asset: asset.publicKey,
        collection: null,
        payer: anchor.getProvider().publicKey,
        owner: null,
        updateAuthority: null,
        logWrapper: null,
      })
      .signers([asset])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can create a Collection", async () => {
    const collection = anchor.web3.Keypair.generate();
    // Add your test here.
    const tx = await program.methods
      .createCollectionV1({
        name: "Hello Anchor!",
        uri: "www.example.com",
        plugins: [],
      })
      .accounts({
        collection: collection.publicKey,
        payer: anchor.getProvider().publicKey,
        updateAuthority: null,
      })
      .signers([collection])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can transfer an Asset", async () => {
    const asset = anchor.web3.Keypair.generate();
    // Add your test here.
    await program.methods
      .createV1({
        name: "Hello Anchor!",
        uri: "www.example.com",
        plugins: null,
      })
      .accounts({
        asset: asset.publicKey,
        collection: null,
        payer: anchor.getProvider().publicKey,
        owner: null,
        updateAuthority: null,
        logWrapper: null,
      })
      .signers([asset])
      .rpc();

    const tx = await program.methods
      .transferV1({})
      .accounts({
        asset: asset.publicKey,
        collection: null,
        payer: anchor.getProvider().publicKey,
        newOwner: anchor.web3.Keypair.generate().publicKey,
        systemProgram: null,
        logWrapper: null,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
  it("Can Add a Plugin to an Asset", async () => {
    const asset = anchor.web3.Keypair.generate();
    // Add your test here.
    await program.methods
      .createV1({
        name: "Hello Anchor!",
        uri: "www.example.com",
        plugins: null,
      })
      .accounts({
        asset: asset.publicKey,
        collection: null,
        payer: anchor.getProvider().publicKey,
        owner: null,
        updateAuthority: null,
        logWrapper: null,
      })
      .signers([asset])
      .rpc();

    const tx = await program.methods
      .addPluginV1({
        name: "Plugin",
        uri: "www.example.com",
        initAuthority: {
          address: {
            address: anchor.getProvider().publicKey,
          },
        },
        plugin: {
          freezeDelegate: [
            {
              frozen: true,
            },
          ],
        },
      })
      .accounts({
        asset: asset.publicKey,
        authority: anchor.getProvider().publicKey,
        collection: null,
        payer: anchor.getProvider().publicKey,
        logWrapper: null,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
