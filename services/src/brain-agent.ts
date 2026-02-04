import { Connection, PublicKey, Keypair, TransactionInstruction } from "@solana/web3.js";
import { loadEnv } from "./config.js";
import fetch from "node-fetch";

const config = loadEnv();
const connection = new Connection(config.solanaRpcUrl);

interface ClaudeRequest {
  model: string;
  max_tokens: number;
  messages: Array<{
    role: string;
    content: string;
  }>;
}

interface ClaudeResponse {
  content: Array<{
    type: string;
    text: string;
  }>;
}

async function analyzeMarket(treasuryBalance: number): Promise<string> {
  const prompt = `You are an AI brain managing a Solana token treasury. 
Current treasury balance: ${treasuryBalance} SOL.
Your role is to:
1. Identify potential AI agents to partner with
2. Suggest capital allocation strategies
3. Recommend when to rebalance or hedge

Provide your analysis in JSON format with:
{
  "recommendation": "string",
  "suggested_partners": ["agent1", "agent2"],
  "allocation": {"agent1": 30, "agent2": 20},
  "confidence": 0.85
}`;

  const requestBody: ClaudeRequest = {
    model: "claude-3-5-sonnet-20241022",
    max_tokens: 1024,
    messages: [
      {
        role: "user",
        content: prompt,
      },
    ],
  };

  const response = await fetch("https://api.anthropic.com/v1/messages", {
    method: "POST",
    headers: {
      "x-api-key": config.claudeApiKey,
      "anthropic-version": "2023-06-01",
      "content-type": "application/json",
    },
    body: JSON.stringify(requestBody),
  });

  if (!response.ok) {
    throw new Error(`Claude API error: ${response.statusText}`);
  }

  const data = (await response.json()) as ClaudeResponse;
  const textContent = data.content.find((c) => c.type === "text");
  if (!textContent) {
    throw new Error("No text response from Claude");
  }

  return textContent.text;
}

async function getTreasuryBalance(): Promise<number> {
  try {
    const treasuryPubkey = new PublicKey(config.treasuryWallet);
    const balance = await connection.getBalance(treasuryPubkey);
    return balance / 1e9; // Convert lamports to SOL
  } catch (error) {
    console.error("Error getting treasury balance:", error);
    return 0;
  }
}

async function submitProposal(
  proposal: string
): Promise<{ success: boolean; transactionId?: string }> {
  try {
    // Parse the proposal to extract partner info
    const proposal_data = JSON.parse(proposal);

    console.log("Proposal submitted to Guardian program:");
    console.log(JSON.stringify(proposal_data, null, 2));

    // In production, this would submit a transaction to the Guardian program
    return {
      success: true,
      transactionId: "mock_tx_id_" + Date.now(),
    };
  } catch (error) {
    console.error("Error submitting proposal:", error);
    return { success: false };
  }
}

async function main() {
  console.log("🧠 Brain AI Agent started...");
  console.log(`Treasury Wallet: ${config.treasuryWallet}`);

  // Monitor treasury every 30 minutes
  const MONITOR_INTERVAL = 30 * 60 * 1000; // 30 minutes
  const MIN_TREASURY_THRESHOLD = 0.5; // 0.5 SOL

  async function checkAndPropose() {
    try {
      const balance = await getTreasuryBalance();
      console.log(`\n📊 Treasury Balance: ${balance.toFixed(4)} SOL`);

      if (balance >= MIN_TREASURY_THRESHOLD) {
        console.log("💡 Analyzing market opportunities...");
        const analysis = await analyzeMarket(balance);

        console.log("📈 AI Analysis:");
        console.log(analysis);

        const proposal = analysis;
        const result = await submitProposal(proposal);

        if (result.success) {
          console.log(`✅ Proposal submitted: ${result.transactionId}`);
        } else {
          console.log("❌ Failed to submit proposal");
        }
      } else {
        console.log(
          `⏳ Insufficient treasury balance (${MIN_TREASURY_THRESHOLD} SOL required)`
        );
      }
    } catch (error) {
      console.error("Error in check cycle:", error);
    }
  }

  // Run immediately
  await checkAndPropose();

  // Schedule regular checks
  setInterval(checkAndPropose, MONITOR_INTERVAL);

  console.log(
    `\n🔄 Scheduled to check treasury every 30 minutes starting now...`
  );
}

main().catch(console.error);
