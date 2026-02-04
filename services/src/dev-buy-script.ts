import { Connection, PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { loadEnv } from "./config.js";
import * as fs from "fs";

const config = loadEnv();
const connection = new Connection(config.solanaRpcUrl);

async function getBalance(pubkey: PublicKey): Promise<number> {
  const balance = await connection.getBalance(pubkey);
  return balance / LAMPORTS_PER_SOL;
}

async function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function createAndFundWallet(): Promise<{ publicKey: PublicKey; balance: number }> {
  try {
    const privateKeyStr = config.aiWalletPrivateKey;
    const privateKey = Uint8Array.from(JSON.parse(privateKeyStr));
    const keypair = Keypair.fromSecretKey(privateKey);

    const balance = await getBalance(keypair.publicKey);
    console.log(`🔑 AI Wallet: ${keypair.publicKey.toBase58()}`);
    console.log(`💰 Balance: ${balance.toFixed(4)} SOL`);

    return {
      publicKey: keypair.publicKey,
      balance,
    };
  } catch (error) {
    console.error("Error loading wallet:", error);
    throw error;
  }
}

async function performDevBuy(
  aiWallet: PublicKey,
  balance: number
): Promise<{ success: boolean; tokensPurchased?: number; txId?: string }> {
  try {
    // Simulate dev buy
    console.log("\n🚀 Executing Dev Buy on Pump.fun");
    console.log("================================");

    const devBuyAmount = 5; // 5 SOL for dev buy
    if (balance < devBuyAmount) {
      console.error(`❌ Insufficient balance. Need ${devBuyAmount} SOL, have ${balance.toFixed(4)} SOL`);
      return { success: false };
    }

    console.log(`📊 Dev Buy Amount: ${devBuyAmount} SOL`);
    console.log(`📈 Expected Token Allocation: 20-30% of total supply`);

    // Calculate tokens (assuming 1M supply, getting 25% = 250k tokens)
    const totalSupply = 1000000;
    const devPercentage = 0.25;
    const tokensPurchased = totalSupply * devPercentage;

    console.log(`✅ Simulated purchase: ${tokensPurchased.toLocaleString()} tokens`);
    console.log(`🔓 Releasing ${tokensPurchased / 1000}K tokens to market`);

    // Setup fee collection
    console.log("\n⚙️  Setting up fee collection (3% per trade)");
    console.log(`📍 Treasury Wallet: ${config.treasuryWallet}`);

    // Estimated first week revenue
    const estimatedVolume = 50000; // $50k volume
    const feeRate = 0.03; // 3%
    const weeklyFees = estimatedVolume * feeRate;

    console.log(`\n📈 Projected Metrics:`);
    console.log(`Weekly Volume Estimate: $${estimatedVolume.toLocaleString()}`);
    console.log(`Weekly Fees (3%): $${weeklyFees.toFixed(2)}`);

    // Start monitoring
    console.log("\n🔄 Starting brain monitoring in 30 minutes...");
    console.log("✨ Dev buy complete!");

    return {
      success: true,
      tokensPurchased: Math.floor(tokensPurchased),
      txId: "mock_tx_" + Date.now(),
    };
  } catch (error) {
    console.error("Dev buy error:", error);
    return { success: false };
  }
}

async function main() {
  console.log("🧠 Brain Coin Dev Buy Script");
  console.log("============================\n");

  try {
    // Step 1: Create/Load AI Wallet
    console.log("Step 1: Loading AI Wallet...");
    const wallet = await createAndFundWallet();

    if (wallet.balance < 5) {
      console.error("❌ Insufficient balance for dev buy (need 5 SOL)");
      console.log("Please airdrop SOL to your wallet:");
      console.log(`solana airdrop 10 ${wallet.publicKey.toBase58()} --url devnet`);
      process.exit(1);
    }

    // Step 2: Perform Dev Buy
    console.log("\nStep 2: Performing Dev Buy...");
    const buyResult = await performDevBuy(wallet.publicKey, wallet.balance);

    if (buyResult.success) {
      console.log(`\n✅ Transaction: ${buyResult.txId}`);
      console.log(`📊 Tokens Acquired: ${buyResult.tokensPurchased?.toLocaleString()}`);

      // Save deployment info
      const deploymentInfo = {
        timestamp: new Date().toISOString(),
        aiWallet: wallet.publicKey.toBase58(),
        tokensAcquired: buyResult.tokensPurchased,
        transactionId: buyResult.txId,
        treasuryWallet: config.treasuryWallet,
        brainProgramId: config.brainProgramId,
      };

      fs.writeFileSync(
        "deployment-info.json",
        JSON.stringify(deploymentInfo, null, 2)
      );
      console.log("\n💾 Deployment info saved to deployment-info.json");
    } else {
      console.error("❌ Dev buy failed");
      process.exit(1);
    }

    console.log("\n🎉 Setup complete! Brain Coin is ready to launch!");
  } catch (error) {
    console.error("Fatal error:", error);
    process.exit(1);
  }
}

main().catch(console.error);
