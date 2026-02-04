# Brain Coin Deployment Guide - Devnet

## Overview
This guide walks through deploying Brain Coin to Solana devnet for testing and verification.

## Prerequisites
- ✅ Node.js v18+
- ✅ Rust 1.70+
- ✅ Solana CLI 1.18+
- ✅ Anchor CLI 0.30.1
- ✅ Solana wallet with some devnet SOL

## Phase 1: Build Smart Contracts

### Step 1: Build all programs
```bash
cd /path/to/braincoin
cargo build --release
```

Expected output:
- `target/release/brain.so`
- `target/release/guardian.so`
- `target/release/agent_syndicate.so`
- `target/release/fee_collector.so`

### Step 2: Verify build artifacts
```bash
ls -la target/release/*.so
```

## Phase 2: Deploy to Devnet

### Step 1: Configure Solana CLI
```bash
solana config set --url https://api.devnet.solana.com
solana config set --keypair ~/.config/solana/id.json
```

### Step 2: Airdrop devnet SOL
```bash
solana airdrop 10
solana balance
```

### Step 3: Deploy Brain Program
```bash
solana program deploy --program-id ~/.config/solana/brain-keypair.json target/release/brain.so
```

Save the program ID from output.

### Step 4: Deploy Guardian Program
```bash
solana program deploy --program-id ~/.config/solana/guardian-keypair.json target/release/guardian.so
```

### Step 5: Deploy Agent Syndicate Program
```bash
solana program deploy --program-id ~/.config/solana/agent-syndicate-keypair.json target/release/agent_syndicate.so
```

### Step 6: Deploy Fee Collector Program
```bash
solana program deploy --program-id ~/.config/solana/fee-collector-keypair.json target/release/fee_collector.so
```

### Step 7: Record Program IDs
Save the four program IDs in `.env`:
```
BRAIN_PROGRAM_ID=<program_id_from_step_3>
GUARDIAN_PROGRAM_ID=<program_id_from_step_4>
AGENT_SYNDICATE_PROGRAM_ID=<program_id_from_step_5>
FEE_COLLECTOR_PROGRAM_ID=<program_id_from_step_6>
```

## Phase 3: Test Integration

### Step 1: Setup environment
```bash
cp .env.example .env
# Edit .env with your actual values and program IDs
```

### Step 2: Create Pump.fun token (mock)
For devnet, create a test token:
```bash
# Create a test SPL token
spl-token create-token

# Record the token mint address in your config
```

### Step 3: Test Brain Agent
```bash
cd services
npm install
npm run brain-agent
```

Expected output:
- ✅ Treasury balance check
- ✅ AI analysis via Claude API
- ✅ Partnership proposal generation

### Step 4: Test Reward Distributor
```bash
npm run reward-distributor
```

Expected output:
- ✅ Holder snapshot generation
- ✅ Reward calculation with tier multipliers
- ✅ Distribution summary

### Step 5: Test Dev Buy
```bash
npm run dev-buy
```

Expected output:
- ✅ Wallet creation
- ✅ Dev buy execution (50% of tokens to AI)
- ✅ Fee collection setup
- ✅ Deployment info saved

## Verification Checklist

### On-Chain Verification
```bash
# Verify Brain program state
solana program show <BRAIN_PROGRAM_ID>

# Verify Guardian program state
solana program show <GUARDIAN_PROGRAM_ID>

# Check transaction history
solana confirmed-transaction <TX_ID>
```

### Account State Verification
```bash
# Check if accounts were created
solana account <ACCOUNT_ADDRESS>

# View account data
solana account <ACCOUNT_ADDRESS> --output json
```

## Troubleshooting

### Build Failures
```bash
# Clean build
cargo clean
cargo build --release

# Check dependencies
cargo tree
```

### Deployment Failures
```bash
# Check program buffer
solana program dump <PROGRAM_ID>

# Verify enough balance
solana balance

# Check program output
solana logs <PROGRAM_ID>
```

### Runtime Issues
```bash
# View detailed logs
RUST_LOG=debug anchor test

# Test individual program
cd programs/brain
cargo build
```

## Next Steps

1. ✅ All 4 programs deployed to devnet
2. ✅ Program IDs recorded in .env
3. ✅ Integration tests passed
4. 🔄 Ready for mainnet deployment

## Mainnet Deployment (When Ready)

See `MAINNET_DEPLOYMENT.md` for production deployment steps.
