import * as fs from "fs";
import * as path from "path";

interface Config {
  solanaRpcUrl: string;
  solanaWSUrl: string;
  aiWalletPrivateKey: string;
  treasuryWallet: string;
  claudeApiKey: string;
  brainProgramId: string;
  guardianProgramId: string;
  agentSyndicateProgramId: string;
  feeCollectorProgramId: string;
}

export function loadEnv(): Config {
  // Load from .env file
  const envPath = path.join(process.cwd(), ".env");
  if (fs.existsSync(envPath)) {
    const envContent = fs.readFileSync(envPath, "utf-8");
    const envVars = envContent.split("\n").reduce((acc, line) => {
      const [key, value] = line.split("=");
      if (key && value) {
        acc[key.trim()] = value.trim();
      }
      return acc;
    }, {} as Record<string, string>);

    Object.entries(envVars).forEach(([key, value]) => {
      process.env[key] = value;
    });
  }

  return {
    solanaRpcUrl: process.env.SOLANA_RPC_URL || "https://api.devnet.solana.com",
    solanaWSUrl: process.env.SOLANA_WS_URL || "wss://api.devnet.solana.com",
    aiWalletPrivateKey: process.env.AI_WALLET_PRIVATE_KEY || "",
    treasuryWallet: process.env.TREASURY_WALLET || "",
    claudeApiKey: process.env.CLAUDE_API_KEY || "",
    brainProgramId: process.env.BRAIN_PROGRAM_ID || "",
    guardianProgramId: process.env.GUARDIAN_PROGRAM_ID || "",
    agentSyndicateProgramId: process.env.AGENT_SYNDICATE_PROGRAM_ID || "",
    feeCollectorProgramId: process.env.FEE_COLLECTOR_PROGRAM_ID || "",
  };
}

export function validateConfig(config: Config): void {
  const required = [
    "aiWalletPrivateKey",
    "treasuryWallet",
    "claudeApiKey",
    "brainProgramId",
    "guardianProgramId",
    "agentSyndicateProgramId",
    "feeCollectorProgramId",
  ];

  for (const field of required) {
    if (!config[field as keyof Config]) {
      throw new Error(`Missing required configuration: ${field}`);
    }
  }
}
