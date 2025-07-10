import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { beforeEach } from "mocha";
import { assert } from "chai";

describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.vault as Program<Vault>;

  const owner = anchor.web3.Keypair.generate();

  const [vaultState] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), owner.publicKey.toBuffer()],
    program.programId
  );

  const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultState.toBuffer()],
    program.programId
  );

  beforeEach(async () => {
    const tx = await provider.connection.requestAirdrop(
      owner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(tx);
    // console.log(tx);
  });

  it("Initialize Vault!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accountsPartial({
        owner: owner.publicKey,
        vault,
        vaultState,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Deposit in vault!", async () => {
    const amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
    await program.methods
      .deposit(amount)
      .accountsPartial({
        owner: owner.publicKey,
        vault,
        vaultState,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    let balance = await provider.connection.getBalance(vault);
    console.log(balance); // 1.000890880 sol
    assert(balance > amount.toNumber());
  });

  // it("Withdraw amount and rent!", async () => {
  //   const amount = new anchor.BN(1000890880);
  //   await program.methods
  //     .withdraw(amount)
  //     .accountsPartial({
  //       owner: owner.publicKey,
  //       vault,
  //       vaultState,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .signers([owner])
  //     .rpc();

  //   let balance = await provider.connection.getBalance(vault);
  //   assert(balance == 0);
  // });

  it("Withdraw amount !", async () => {
    const amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
    await program.methods
      .withdraw(amount)
      .accountsPartial({
        owner: owner.publicKey,
        vault,
        vaultState,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();
  });

  it("Close", async () => {
    await program.methods
      .close()
      .accountsPartial({
        owner: owner.publicKey,
        vault,
        vaultState,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    const balance = await provider.connection.getBalance(vault);
    assert(balance == 0);
  });
});
