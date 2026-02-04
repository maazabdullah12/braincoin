# 🧠 Brain Coin - AI-Powered Token with Pump.fun Integration

An autonomous Solana token system where an AI brain automatically manages treasury, partners with other AI agents, and distributes profits to holders.

## 🎯 Vision

Brain Coin creates a self-governing token where:
- **AI Brain** monitors treasury and makes strategic decisions
- **Smart Contracts** enforce governance and multi-sig approvals
- **Agent Network** partners with other AI agents for profit generation
- **Token Holders** earn rewards through fee distribution and tier-based multipliers

## 🏗️ Architecture

### Smart Contracts (4 Anchor Programs)

#### 1. **Brain Program** (`programs/brain/`)
- Holds token mint authority
- Manages brain state (PnL, evolution tier)
- Evolution tiers: Cellular → Sentient → Superintelligent → Godlike
- Circuit breaker: auto-freeze if PnL < -30%

#### 2. **Guardian Program** (`programs/guardian/`)
- Governance and voting system
- 48-hour timelock for community review
- 2-of-3 multi-sig approval
- Distributes rewards to holders

#### 3. **Agent Syndicate Program** (`programs/agent-syndicate/`)
- Manages AI agent partnerships
- Tracks capital allocation per partner
- Calculates rewards with staking tier multipliers:
  - Bronze: 1x
  - Silver: 1.5x
  - Gold: 2x
  - Platinum: 3x

#### 4. **Fee Collector Program** (`programs/fee-collector/`)
- Collects 3-5% trading fees
- Routes to treasury wallet
- Tracks total fees and withdrawals

### Off-Chain Services (TypeScript/Node.js)

#### Brain Agent (`services/src/brain-agent.ts`)
- Monitors treasury every 30 minutes
- Uses Claude API for AI analysis
- Generates partnership proposals
- Submits to Guardian for voting

#### Reward Distributor (`services/src/reward-distributor.ts`)
- Calculates weekly rewards
- Applies tier multipliers
- Auto-compounds for Platinum holders
- Tracks holder claims

#### Dev Buy Script (`services/src/dev-buy-script.ts`)
- Executes initial 5 SOL buy
- Acquires 20-30% of supply
- Sets up fee collection
- Starts monitoring

## 📋 Project Structure

```
braincoin/
├── programs/
│   ├── brain/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── guardian/
│   ├── agent-syndicate/
│   └── fee-collector/
├── services/
│   ├── src/
│   │   ├── brain-agent.ts
│   │   ├── reward-distributor.ts
│   │   ├── dev-buy-script.ts
│   │   └── config.ts
│   ├── package.json
│   └── tsconfig.json
├── tests/
├── Cargo.toml
├── Anchor.toml
├── .env.example
├── DEVNET_DEPLOYMENT.md
├── MAINNET_DEPLOYMENT.md
└── README.md
```

## 🚀 Quick Start

### Prerequisites
```bash
# Install required tools
node --version  # v18+
rustc --version # 1.70+
solana --version # 1.18+
anchor --version # 0.30.1+
```

### Build Contracts
```bash
cd /path/to/braincoin
cargo build --release
```

### Setup Environment
```bash
cp .env.example .env
# Edit .env with your values:
# - AI_WALLET_PRIVATE_KEY
# - TREASURY_WALLET
# - CLAUDE_API_KEY
# - Program IDs (after deployment)
```

### Install Services
```bash
cd services
npm install
```

### Deploy to Devnet
See `DEVNET_DEPLOYMENT.md` for step-by-step instructions.

## 💻 Running Services

### Brain Agent
```bash
cd services
npm run brain-agent
```
- Monitors treasury balance
- Analyzes market with Claude API
- Generates partnership proposals
- Submits to Guardian program

### Reward Distributor
```bash
npm run reward-distributor
```
- Calculates weekly rewards
- Applies tier multipliers
- Processes holder claims
- Auto-compounds for high tiers

### Dev Buy
```bash
npm run dev-buy
```
- Executes initial token purchase
- Sets up fee collection
- Starts treasury monitoring

## 📊 Key Features

### Smart Contract Features
- ✅ Token-2022 mint with metadata
- ✅ Evolution tier system (4 tiers)
- ✅ Circuit breaker protection (-30% PnL)
- ✅ Multi-sig governance (2-of-3)
- ✅ 48-hour timelock voting
- ✅ Partnership tracking
- ✅ Profit reporting
- ✅ Staking tier multipliers
- ✅ Fee collection mechanism

### AI Features
- ✅ Real-time treasury monitoring
- ✅ Claude API integration
- ✅ Partnership opportunity scanning
- ✅ Proposal generation
- ✅ Reward calculation
- ✅ Auto-compounding

## 🔄 Deployment Flow

### Phase 1: Devnet Testing (FREE)
1. Build all 4 contracts
2. Deploy to devnet
3. Create test Pump.fun token
4. Run integration tests
5. Verify fee collection
6. Test reward distribution

### Phase 2: Mainnet Deployment (REAL MONEY)
1. Deploy all 4 contracts
2. Create real Pump.fun token (~2 SOL)
3. Fund AI wallet with 5 SOL
4. Execute dev buy
5. Start AI brain monitoring
6. Launch to community

See deployment guides:
- `DEVNET_DEPLOYMENT.md` - Testing checklist
- `MAINNET_DEPLOYMENT.md` - Production deployment

## 🛡️ Safety Features

### Circuit Breaker
- Automatic brain freeze if PnL < -30%
- Prevents catastrophic losses
- Admin can unfreeze when recovered

### Multi-Sig Governance
- 2-of-3 approval required
- 48-hour timelock for visibility
- Community review period

### Staking Tiers
- Incentivizes long-term holding
- Multiplier rewards: 1x to 3x
- Platinum tier auto-compounds

## 📈 Revenue Model

1. **Trading Fees**: 3-5% on all trades
2. **Accumulated in Treasury**
3. **AI Makes Partnerships**
4. **Profits Distributed to Holders**
5. **Weekly Rewards + Tier Multipliers**

### Example Economics
- Volume: $50k/week
- Fee (3%): $1,500/week
- Holder rewards: $900 (60% of fees)
- Treasury growth: $600/week

## 🧪 Testing

### Test Individual Programs
```bash
cd programs/brain
cargo test --release
```

### Test Entire Workspace
```bash
cargo test --release
```

### Test TypeScript Services
```bash
cd services
npm run dev  # Run dev environment
```

## 📚 Documentation

- `DEVNET_DEPLOYMENT.md` - Devnet deployment steps
- `MAINNET_DEPLOYMENT.md` - Mainnet deployment guide
- `.env.example` - Configuration template
- `programs/brain/src/lib.rs` - Brain program docs
- `programs/guardian/src/lib.rs` - Governance docs
- `programs/agent-syndicate/src/lib.rs` - Syndicate docs
- `programs/fee-collector/src/lib.rs` - Fee collection docs

## 🔧 Configuration

### .env Variables
```
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_WS_URL=wss://api.devnet.solana.com
AI_WALLET_PRIVATE_KEY=[...]
TREASURY_WALLET=...
CLAUDE_API_KEY=sk-ant-...
BRAIN_PROGRAM_ID=...
GUARDIAN_PROGRAM_ID=...
AGENT_SYNDICATE_PROGRAM_ID=...
FEE_COLLECTOR_PROGRAM_ID=...
```

### Monitoring Intervals
- Brain monitoring: every 30 minutes
- Weekly rewards: every 7 days
- Fee withdrawal: daily (configurable)

## 🐛 Troubleshooting

### Build Errors
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Check Rust version
rustc --version  # Should be 1.70+
```

### Deployment Issues
```bash
# Verify Solana connection
solana ping

# Check balance
solana balance

# View recent logs
solana logs <PROGRAM_ID>
```

### Runtime Issues
```bash
# Enable debug logging
RUST_LOG=debug npm run brain-agent

# Check transaction details
solana transaction <TX_ID>
```

## 📊 Monitoring

### Track Treasury
```bash
solana account <TREASURY_WALLET> --output json
```

### Monitor Rewards
```bash
# Check reward distribution logs
tail -f logs/rewards.log
```

### View Partner Performance
```bash
# Query agent syndicate program state
npm run check-partners
```

## 🤝 Contributing

Brain Coin is open for enhancements:
1. Test your changes on devnet
2. Ensure all tests pass
3. Update documentation
4. Submit for review

## ⚖️ License

MIT License - See LICENSE file

## 🚨 Disclaimer

This is a complex DeFi system involving real blockchain value. Use with caution:
- Test thoroughly on devnet first
- Start with small amounts
- Monitor closely after launch
- Keep private keys secure
- Have an emergency shutdown plan

## 📞 Support

- Issues: Check logs in `logs/` directory
- Errors: Review Solana docs at https://docs.solana.com
- Questions: See deployment guides above
- Emergency: See MAINNET_DEPLOYMENT.md emergency procedures

## 🎯 Roadmap

### Phase 1: MVP (Current)
- ✅ 4 smart contracts
- ✅ Basic AI integration
- ✅ Devnet testing

### Phase 2: Production
- Deploy to mainnet
- Launch on Pump.fun
- Community governance

### Phase 3: Expansion
- Advanced AI strategies
- Cross-chain partnerships
- Advanced governance features

---

**Built with ❤️ for the Solana ecosystem**
