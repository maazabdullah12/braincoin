# Brain Coin Deployment Guide - Mainnet

## ⚠️ Important: Mainnet is Live with Real Money

Only proceed if you have:
- ✅ Tested extensively on devnet
- ✅ Audited smart contracts (optional but recommended)
- ✅ Real SOL for deployment + dev buy
- ✅ Budget for Pump.fun token creation (~2 SOL)

## Phase 1: Mainnet Setup

### Step 1: Configure for Mainnet
```bash
solana config set --url https://api.mainnet-beta.solana.com
solana config set --keypair ~/.config/solana/mainnet-id.json
```

### Step 2: Verify mainnet wallet
```bash
solana balance
solana address
```

Required balance:
- Deployment costs: ~2 SOL
- Dev buy: 5 SOL
- Gas buffer: 1 SOL
- **Total: ~8 SOL minimum**

## Phase 2: Deploy to Mainnet

### Step 1: Update configuration
```bash
# Update Anchor.toml
[provider]
cluster = "Mainnet"
```

### Step 2: Deploy Brain Program
```bash
solana program deploy \
  --program-id ~/.config/solana/brain-keypair.json \
  target/release/brain.so \
  --keypair ~/.config/solana/mainnet-id.json

# Record program ID
export BRAIN_PROGRAM_ID=<id>
```

### Step 3: Deploy Guardian Program
```bash
solana program deploy \
  --program-id ~/.config/solana/guardian-keypair.json \
  target/release/guardian.so \
  --keypair ~/.config/solana/mainnet-id.json

export GUARDIAN_PROGRAM_ID=<id>
```

### Step 4: Deploy Agent Syndicate Program
```bash
solana program deploy \
  --program-id ~/.config/solana/agent-syndicate-keypair.json \
  target/release/agent_syndicate.so \
  --keypair ~/.config/solana/mainnet-id.json

export AGENT_SYNDICATE_PROGRAM_ID=<id>
```

### Step 5: Deploy Fee Collector Program
```bash
solana program deploy \
  --program-id ~/.config/solana/fee-collector-keypair.json \
  target/release/fee_collector.so \
  --keypair ~/.config/solana/mainnet-id.json

export FEE_COLLECTOR_PROGRAM_ID=<id>
```

## Phase 3: Create Pump.fun Token

### Option 1: Via Pump.fun Website
1. Visit https://pump.fun
2. Create a new token
3. Token name: "Brain Coin" (BRAIN)
4. Total supply: 1,000,000 BRAIN
5. Pay ~2 SOL transaction fee
6. Record mint address

### Option 2: Via CLI
```bash
# Create SPL token
spl-token create-token \
  --mint-account ~/.config/solana/brain-token.json

# Get mint address
solana address -k ~/.config/solana/brain-token.json
```

## Phase 4: Initialize Programs

### Step 1: Create Associated Token Accounts
```bash
spl-token create-account <BRAIN_MINT>
spl-token create-account <BRAIN_MINT> --owner <TREASURY_WALLET>
```

### Step 2: Initialize Brain State
```bash
# This would be done via anchor typescript client
# See services/src/initialize.ts (create this file)

cd services
npm install
npm run initialize-mainnet
```

## Phase 5: Mainnet Dev Buy

### Step 1: Update .env for mainnet
```bash
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SOLANA_WS_URL=wss://api.mainnet-beta.solana.com
# ... other vars
```

### Step 2: Execute dev buy
```bash
npm run dev-buy
```

This will:
- Buy 20-30% of supply with 5 SOL
- Set up fee collection
- Start treasury monitoring
- Begin AI brain operations

## Phase 6: Launch Community

### Step 1: Announce token
- Twitter announcement
- Discord server setup
- Website launch
- Documentation

### Step 2: Start bot operations
```bash
# Brain agent monitors treasury
npm run brain-agent

# Reward distributor runs weekly
npm run reward-distributor
```

### Step 3: Monitor metrics
```bash
# Watch for:
# - Trading volume
# - Fee accumulation
# - Holder count
# - Partnership proposals

tail -f logs/brain-agent.log
```

## Mainnet Verification

### Confirm Deployments
```bash
# Check program upgrade authority
solana program show <PROGRAM_ID>

# Verify buffer
solana program dump <PROGRAM_ID>

# Check transaction
solana confirmed-transaction <TX_ID>
```

### Monitor Treasury
```bash
solana account <TREASURY_ADDRESS>
solana account-info <TREASURY_ADDRESS> --output json
```

### Verify Token
```bash
spl-token supply <BRAIN_MINT>
spl-token accounts <BRAIN_MINT>
spl-token supply <BRAIN_MINT> --owner <TREASURY_ADDRESS>
```

## Post-Launch Operations

### Daily Tasks
1. Monitor treasury balance
2. Review AI partnership proposals
3. Check trading volume and fees

### Weekly Tasks
1. Distribute rewards to holders
2. Rebalance treasury if needed
3. Review AI tier evolution

### Monthly Tasks
1. Governance review
2. Partner performance evaluation
3. Strategic planning

## Emergency Procedures

### Circuit Breaker Triggered
If PnL drops below -30%:
```bash
# Brain automatically freezes
# Admin can unfreeze with:
npm run unfreeze-brain
```

### Pause Operations
```bash
# Freeze all AI operations
solana program set-upgrade-authority <PROGRAM_ID> --new-upgrade-authority <KEY>
```

### Graceful Shutdown
```bash
# Stop services
pkill brain-agent
pkill reward-distributor

# Archive logs
tar -czf logs-$(date +%Y%m%d).tar.gz logs/
```

## Success Metrics

- ✅ 4 programs deployed and verified
- ✅ Token created and distributed
- ✅ AI brain monitoring actively
- ✅ Fees accumulating in treasury
- ✅ Weekly rewards distributed
- ✅ Community engaged

## Resources

- [Solana Docs](https://docs.solana.com)
- [Anchor Docs](https://www.anchor-lang.com)
- [Pump.fun](https://pump.fun)
- [SPL Token Docs](https://spl.solana.com/token)

## Support

For issues:
1. Check logs: `logs/`
2. Review errors in detail
3. Consult Solana documentation
4. Run devnet tests to debug
5. Contact Solana support if needed
