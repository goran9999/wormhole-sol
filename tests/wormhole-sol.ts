import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  Keypair,
  PublicKey,
  Transaction,
  SYSVAR_CLOCK_PUBKEY,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
} from "@solana/web3.js";
import { IDL, WormholeSol } from "../target/types/wormhole_sol";

import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
describe("wormhole-sol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.WormholeSol as Program<WormholeSol>;
  const wallet = Keypair.fromSecretKey(new Uint8Array([10]));

  const connection = anchor.getProvider().connection;

  const wormholeSolProgram = new Program<WormholeSol>(
    IDL,
    "8QS89n567dJk829iueJdxFUeqFkVjRBPU3w674BGmB8m",
    new anchor.AnchorProvider(connection, new NodeWallet(wallet), {
      commitment: "confirmed",
    })
  );

  const wormholeBridgeContract = new PublicKey(
    "Bridge1p5gheXUvJ6jGWGeCsgPKgnE3YgdGKRVCMY9o"
  );

  it("Is initialized!", async () => {
    const [wormholeConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from("Bridge")],
      wormholeBridgeContract
    );

    const [feeCollector] = PublicKey.findProgramAddressSync(
      [Buffer.from("fee_collector")],
      wormholeBridgeContract
    );

    const [messageConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from("message-config"), wallet.publicKey.toBuffer()],
      wormholeSolProgram.programId
    );

    let indexBuffer = Buffer.alloc(4);

    try {
      const messageConfigAcc =
        await wormholeSolProgram.account.messageConfig.fetch(messageConfig);

      indexBuffer.writeUint32LE(messageConfigAcc.index + 1, 0);
    } catch (error) {
      indexBuffer.writeUint32LE(1, 0);
    }

    const [sequence] = PublicKey.findProgramAddressSync(
      [Buffer.from("Sequence"), wormholeSolProgram.programId.toBuffer()],
      wormholeBridgeContract
    );

    const [messageKey] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("message"),
        wormholeConfig.toBuffer(),
        wallet.publicKey.toBuffer(),
        indexBuffer,
      ],
      wormholeSolProgram.programId
    );

    console.log(messageKey.toString(), "MESSAGE");

    const ix = await wormholeSolProgram.methods
      .emitWormholeMessage(Buffer.from(""), 12)
      .accounts({
        payer: wallet.publicKey,
        wormholeConfig,
        message: messageKey,
        clock: SYSVAR_CLOCK_PUBKEY,
        messageConifg: messageConfig,
        emitterAddress: new PublicKey(
          "HNooTQjnQsvJYAX6dVEbfGRENWG4wXsvmZA6MZShEg8u"
        ),
        rent: SYSVAR_RENT_PUBKEY,
        wormhole: wormholeBridgeContract,
        systemProgram: SystemProgram.programId,
        feeCollector: feeCollector,
        sequence: new PublicKey("2U9KXipmE7NgD14q8r68sa5hwATp7EVwXoHumbto92HX"),
      })
      .instruction();

    try {
      const tx = new Transaction({
        feePayer: wallet.publicKey,
        recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
      });
      tx.add(ix);

      const txSig = await connection.sendTransaction(tx, [wallet]);
      console.log(txSig);
    } catch (error) {
      console.log(error);
    }
  });
});
