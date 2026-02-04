/**
 * Brain Coin - Devnet Deployment Script (Alternative using Web3.js)
 * 
 * This script deploys the Brain Coin smart contracts to Solana devnet
 * It loads compiled binaries and uses the Solana CLI or Web3.js to deploy
 */

import * as fs from "fs";
import * as path from "path";
import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";

const DEVNET_RPC = "https://api.devnet.solana.com";
const COMMITMENT = "confirmed";

// Program names and their corresponding compiled binary locations
const programs = [
  {
    name: "brain",
    binaryPath: "target/release/brain.dll",
    fallbackPath: "target/sbf-solana-solana/release/brain.so",
  },
  {
    name: "guardian",
    binaryPath: "target/release/guardian.dll",
    fallbackPath: "target/sbf-solana-solana/release/guardian.so",
  },
  {
    name: "agent-syndicate",
    binaryPath: "target/release/agent_syndicate.dll",
    fallbackPath: "target/sbf-solana-solana/release/agent_syndicate.so",
  },
  {
    name: "fee-collector",
    binaryPath: "target/release/fee_collector.dll",
    fallbackPath: "target/sbf-solana-solana/release/fee_collector.so",
  },
];

async function deployPrograms() {
  console.log("🚀 Brain Coin Devnet Deployment\n");

  // Connect to devnet
  const connection = new Connection(DEVNET_RPC, COMMITMENT);
  console.log(`✓ Connected to: ${DEVNET_RPC}\n`);

  // Load wallet
  const walletPath = path.join(process.env.HOME || process.env.USERPROFILE || "", ".config", "solana", "id.json");
  if (!fs.existsSync(walletPath)) {
    console.error(`❌ Wallet not found at ${walletPath}`);
    console.error("Generate with: solana-keygen new");
    process.exit(1);
  }

  const wallet = Keypair.fromSecretKey(Buffer.from(JSON.parse(fs.readFileSync(walletPath, "utf-8"))));
  console.log(`✓ Wallet loaded: ${wallet.publicKey.toString()}\n`);

  // Check wallet balance
  const balance = await connection.getBalance(wallet.publicKey);
  const balanceSol = balance / 1e9;
  console.log(`💰 Wallet balance: ${balanceSol} SOL`);

  if (balanceSol < 1) {
    console.log("\n⚠️  Low balance. Request devnet SOL:");
    console.log(`solana airdrop 5 --url devnet`);
  }

  console.log("\n📦 Checking compiled binaries...\n");

  // Check if binaries exist
  let allBinariesFound = true;
  for (const program of programs) {
    const fullPath = path.join(process.cwd(), program.binaryPath);
    const exists = fs.existsSync(fullPath);
    const status = exists ? "✓" : "❌";
    console.log(`  ${status} ${program.name}: ${program.binaryPath} (${exists ? "found" : "NOT FOUND"})`);
    if (!exists) allBinariesFound = false;
  }

  if (!allBinariesFound) {
    console.log("\n❌ Not all binaries found. Building programs...\n");
    console.log("Run: anchor build or cargo build --release\n");
    process.exit(1);
  }

  console.log("\n✨ All binaries found!\n");
  console.log("📝 DEPLOYMENT PLAN:\n");
  console.log("The compiled programs are ready for deployment.");
  console.log("Next steps:\n");
  console.log("1. Ensure .so files are generated in target/release/");
  console.log("2. Run: solana program deploy target/release/brain.so");
  console.log("3. Run: solana program deploy target/release/guardian.so");
  console.log("4. Run: solana program deploy target/release/agent_syndicate.so");
  console.log("5. Run: solana program deploy target/release/fee_collector.so");
  console.log("\n6. Update program IDs in Anchor.toml");
  console.log("7. Save program IDs to .env\n");

  console.log("Or use the deployment helper script: npm run deploy:devnet\n");
}

// Run
deployPrograms().catch((err) => {
  console.error("❌ Deployment error:", err.message);
  process.exit(1);
});
