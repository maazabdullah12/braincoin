import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { loadEnv } from "./config.js";

const config = loadEnv();
const connection = new Connection(config.solanaRpcUrl);

interface HolderSnapshot {
  wallet: string;
  tokenBalance: number;
  stakingTier: "Bronze" | "Silver" | "Gold" | "Platinum";
  claimed: boolean;
}

interface WeeklyReward {
  total_pool: number;
  holders: HolderSnapshot[];
  distribution_timestamp: number;
}

function calculateTierMultiplier(tier: string): number {
  const multipliers: Record<string, number> = {
    Bronze: 1.0,
    Silver: 1.5,
    Gold: 2.0,
    Platinum: 3.0,
  };
  return multipliers[tier] || 1.0;
}

function calculateRewardShare(
  holder: HolderSnapshot,
  totalPool: number,
  totalHolders: number
): number {
  const baseShare = totalPool / totalHolders;
  const multiplier = calculateTierMultiplier(holder.stakingTier);
  return baseShare * multiplier;
}

async function generateWeeklyRewards(
  rewardPool: number
): Promise<WeeklyReward> {
  // Mock holder data - in production this would be fetched from on-chain
  const holders: HolderSnapshot[] = [
    {
      wallet: "holder1...xyz",
      tokenBalance: 1000,
      stakingTier: "Platinum",
      claimed: false,
    },
    {
      wallet: "holder2...abc",
      tokenBalance: 500,
      stakingTier: "Gold",
      claimed: false,
    },
    {
      wallet: "holder3...def",
      tokenBalance: 250,
      stakingTier: "Silver",
      claimed: false,
    },
    {
      wallet: "holder4...ghi",
      tokenBalance: 100,
      stakingTier: "Bronze",
      claimed: false,
    },
  ];

  // Calculate individual rewards with tier multipliers
  const totalMultiplier = holders.reduce((sum, holder) => {
    return sum + calculateTierMultiplier(holder.stakingTier);
  }, 0);

  const holdersWithRewards = holders.map((holder) => ({
    ...holder,
    reward: (rewardPool * calculateTierMultiplier(holder.stakingTier)) / totalMultiplier,
  }));

  console.log("📊 Weekly Reward Distribution:");
  console.log("================================");
  let totalDistributed = 0;
  for (const holder of holdersWithRewards) {
    console.log(
      `Wallet: ${holder.wallet} | Tier: ${holder.stakingTier} | Reward: ${holder.reward.toFixed(4)} BRAIN`
    );
    totalDistributed += holder.reward;
  }
  console.log(`Total Distributed: ${totalDistributed.toFixed(4)} BRAIN`);
  console.log("================================\n");

  return {
    total_pool: rewardPool,
    holders: holdersWithRewards,
    distribution_timestamp: Date.now(),
  };
}

async function claimReward(
  holder: string,
  amount: number
): Promise<{ success: boolean; transactionId?: string }> {
  try {
    console.log(`💰 Processing reward claim for ${holder}: ${amount} BRAIN`);

    // In production, this would submit a transaction to the Guardian program
    return {
      success: true,
      transactionId: "mock_tx_id_" + Date.now(),
    };
  } catch (error) {
    console.error("Error claiming reward:", error);
    return { success: false };
  }
}

async function autoCompound(
  holder: string,
  amount: number
): Promise<{ success: boolean }> {
  try {
    console.log(
      `🔄 Auto-compounding ${amount} BRAIN for ${holder} into staking...`
    );

    // In production, would auto-stake the rewards
    return { success: true };
  } catch (error) {
    console.error("Error in auto-compound:", error);
    return { success: false };
  }
}

async function main() {
  console.log("💸 Reward Distributor started...");
  console.log(`Program ID: ${config.agentSyndicateProgramId}\n`);

  // Generate weekly rewards (assume 100 BRAIN reward pool)
  const weeklyReward = await generateWeeklyRewards(100);

  // Process claims for each holder
  console.log("Processing claims...\n");
  for (const holder of weeklyReward.holders) {
    if (!holder.claimed) {
      const result = await claimReward(holder.wallet, holder.reward);

      if (result.success) {
        console.log(`✅ Claim processed: ${result.transactionId}`);

        // Auto-compound for Platinum tier holders
        if (holder.stakingTier === "Platinum") {
          const compoundResult = await autoCompound(holder.wallet, holder.reward);
          if (compoundResult.success) {
            console.log(`✅ Auto-compounded for ${holder.wallet}`);
          }
        }
      } else {
        console.log(`❌ Claim failed for ${holder.wallet}`);
      }
    }
  }

  console.log("\n✨ Weekly reward distribution complete!");
}

main().catch(console.error);
