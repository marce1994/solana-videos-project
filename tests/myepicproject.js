const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  // Create and set the provider. We set it before but we needed to update it, so that it can communicate with our frontend!
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;
	
  // Create an account keypair for our program to use.
  const baseAccount = anchor.web3.Keypair.generate();

  // Call start_stuff_off, pass it the params it needs!
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  // Fetch data from the account.
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Video Count', account.totalVideos.toString())

  // You'll need to now pass a VIDEO link to the function! You'll also need to pass in the user submitting the Video!
  await program.rpc.addVideo("https://youtu.be/XV3yLxNjXLQ", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  // Access video_list on the account!
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Video List', account.videoList);

  await program.rpc.likeVideo(0, {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  // Access video_list on the account!
  console.log('like');
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Video List', account.videoList);

  await program.rpc.likeVideo(0, {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  // Access video_list on the account!
  console.log('cancel-like');
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Video List', account.videoList);
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();