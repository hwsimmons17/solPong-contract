const anchor = require("@project-serum/anchor");

describe("pong-contract", () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);
  const connection = provider.connection

  const program = anchor.workspace.PongContract;
  let escrowProgram;
  let programAuthorityBump;
  const us = provider.wallet.payer; // anchor.web3.Keypair.generate();

  const cancelPlayer = new anchor.web3.Keypair()
  const playerOne = new anchor.web3.Keypair()
  const playerTwo = new anchor.web3.Keypair()
  let matchPDA
  let matchBump

  console.log(playerOne.publicKey.toBytes())

  before(async ()=>{

  const [_programAuthority, _authorityBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      ["authorityy"],
      program.programId
    );

  escrowProgram = _programAuthority;
  programAuthorityBump = _authorityBump;

  const [_matchPDA, _authorityBumpp] = await anchor.web3.PublicKey.findProgramAddress(
    [playerOne.publicKey.toBytes(), playerTwo.publicKey.toBytes()],program.programId
  )
  matchPDA = _matchPDA
  matchBump = _authorityBumpp

  await provider.connection.requestAirdrop(cancelPlayer.publicKey, 2*anchor.web3.LAMPORTS_PER_SOL)
  await provider.connection.requestAirdrop(playerOne.publicKey, 2*anchor.web3.LAMPORTS_PER_SOL)
  await provider.connection.requestAirdrop(playerTwo.publicKey, 2*anchor.web3.LAMPORTS_PER_SOL)
  

})



  it("Is initialized!", async () => {
    await program.rpc.initialize(
      new anchor.BN(programAuthorityBump),
      {
        accounts: {
          trustedserver: provider.wallet.publicKey,
          escrow: escrowProgram,
          systemProgram: anchor.web3.SystemProgram.programId
        },
      }
    );
  });

  it("Pays the piper", async ()=>{
    provider.connection.fetch
    await program.rpc.paypiper(
      new anchor.BN(programAuthorityBump),
      {
        accounts: {
          trustedserver: provider.wallet.publicKey,
          newplayer: cancelPlayer.publicKey,
          escrow: escrowProgram,
          systemProgram: anchor.web3.SystemProgram.programId
        },
        signers: [cancelPlayer]
      }
    );
  })

  it("cancels the contract", async ()=>{
    await program.rpc.cancel(
      new anchor.BN(programAuthorityBump),
      {
        accounts: {
          trustedserver:provider.wallet.publicKey,
          newplayer: cancelPlayer.publicKey,
          escrow: escrowProgram,
          systemProgram: anchor.web3.SystemProgram.programId
        },
        signers: [cancelPlayer]
      }
    );
  })

  it("adds a player one", async ()=>{
    await program.rpc.paypiper(
      new anchor.BN(programAuthorityBump),
      {
        accounts: {
          trustedserver: provider.wallet.publicKey,
          newplayer: playerOne.publicKey,
          escrow: escrowProgram,
          systemProgram: anchor.web3.SystemProgram.programId
        },
        signers: [playerOne]
      }
    );

   
  })

  it("adds a player two", async ()=>{
    await program.rpc.paypiper(
      new anchor.BN(programAuthorityBump),
      {
        accounts: {
          trustedserver: provider.wallet.publicKey,
          newplayer: playerTwo.publicKey,
          escrow: escrowProgram,
          systemProgram: anchor.web3.SystemProgram.programId
        },
        signers: [playerTwo]
      }
    );
  })

  it("Pairs the players", async()=>{
    await program.rpc.matchplayers(
      new anchor.BN(matchBump),
      {
        accounts: {
          trustedserver: provider.wallet.publicKey,
          playerone: playerOne.publicKey,
          playertwo: playerTwo.publicKey,
          escrow: escrowProgram,
          newescrow: matchPDA,
          systemProgram: anchor.web3.SystemProgram.programId
        },
      }
    );
  })

  it("declares a winner for the game", async ()=>{
    await program.rpc.declarewinner(
      new anchor.BN(matchBump),
      {
        accounts: {
          trustedserver: provider.wallet.publicKey,
          winner: playerTwo.publicKey,
          newescrow: matchPDA,
          escrow: escrowProgram,
          systemProgram: anchor.web3.SystemProgram.programId
        },
      }
    );
  })
});
