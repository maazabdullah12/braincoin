# 🚀 Brain Coin Deployment Checklist

## Pre-Deployment Checklist

### Environment Setup
- [ ] Node.js v18+ installed
- [ ] Rust 1.70+ installed
- [ ] Solana CLI 1.18+ installed
- [ ] Anchor 0.30.1+ installed
- [ ] Git installed and configured
- [ ] VS Code installed with extensions

### Repository Setup
- [ ] Repository cloned
- [ ] Dependencies installed
- [ ] `.env` file created from `.env.example`
- [ ] All configuration values filled in
- [ ] Git initialized and commited

## Build & Compilation

### Compile Smart Contracts
- [ ] `cargo build --release` completes successfully
- [ ] All 4 programs compile without errors:
  - [ ] brain.so
  - [ ] guardian.so
  - [ ] agent-syndicate.so
  - [ ] fee-collector.so
- [ ] No warnings in final output
- [ ] Build time noted: _______ minutes

### Verify Build Artifacts
- [ ] `target/release/brain.so` exists
- [ ] `target/release/guardian.so` exists
- [ ] `target/release/agent_syndicate.so` exists
- [ ] `target/release/fee_collector.so` exists
- [ ] File sizes reasonable (> 1MB each)

## Devnet Deployment

### Solana Configuration
- [ ] Switched to devnet: `solana config set --url https://api.devnet.solana.com`
- [ ] Correct keypair configured
- [ ] Wallet has SOL balance: _________ SOL
- [ ] Airdrop completed if needed

### Deploy Programs
- [ ] Brain program deployed
  - [ ] Program ID recorded: ______________________________
  - [ ] Deployment verified with `solana program show`
  
- [ ] Guardian program deployed
  - [ ] Program ID recorded: ______________________________
  - [ ] Deployment verified
  
- [ ] Agent Syndicate program deployed
  - [ ] Program ID recorded: ______________________________
  - [ ] Deployment verified
  
- [ ] Fee Collector program deployed
  - [ ] Program ID recorded: ______________________________
  - [ ] Deployment verified

### Update Configuration
- [ ] `.env` file updated with program IDs
- [ ] All 4 program IDs in correct format
- [ ] No placeholder values remaining
- [ ] Claude API key valid and tested

## Integration Testing

### Services Setup
- [ ] `cd services && npm install` completed
- [ ] All dependencies installed
- [ ] No security vulnerabilities (`npm audit`)
- [ ] TypeScript compilation successful

### Test Brain Agent
- [ ] Running: `npm run brain-agent`
- [ ] [ ] Connects to RPC successfully
- [ ] [ ] Fetches treasury balance
- [ ] [ ] Calls Claude API (check for errors)
- [ ] [ ] Generates analysis
- [ ] [ ] Submits proposal to Guardian

### Test Reward Distributor
- [ ] Running: `npm run reward-distributor`
- [ ] [ ] Generates holder snapshot
- [ ] [ ] Calculates tier multipliers correctly
- [ ] [ ] Outputs reward distribution
- [ ] [ ] Mock claim transactions created
- [ ] [ ] Auto-compound tested

### Test Dev Buy
- [ ] Running: `npm run dev-buy`
- [ ] [ ] Loads AI wallet
- [ ] [ ] Checks balance (5 SOL+)
- [ ] [ ] Simulates Pump.fun purchase
- [ ] [ ] Calculates token allocation (25%)
- [ ] [ ] Saves deployment-info.json

## Pre-Launch Verification

### Smart Contract Verification
- [ ] Brain program state queryable
- [ ] Guardian program responds to queries
- [ ] Agent Syndicate partnership system works
- [ ] Fee Collector tracks fees correctly
- [ ] No stuck transactions
- [ ] Event logs being emitted

### Integration Verification
- [ ] Services can read program state
- [ ] Services can submit transactions
- [ ] Error handling works correctly
- [ ] Logging configured and working
- [ ] Environment variables validated

### Security Checks
- [ ] No hardcoded secrets in code
- [ ] Private keys stored safely
- [ ] API keys not logged
- [ ] Rate limiting configured
- [ ] Input validation in place
- [ ] Error messages don't leak sensitive data

## Mainnet Preparation

### Documentation
- [ ] DEVNET_DEPLOYMENT.md reviewed
- [ ] MAINNET_DEPLOYMENT.md reviewed
- [ ] All instructions tested on devnet
- [ ] Emergency procedures documented
- [ ] Runbook created for operators

### Financial Preparation
- [ ] Budget allocated:
  - [ ] Deployment: ~3 SOL
  - [ ] Dev buy: 5 SOL
  - [ ] Buffer: 2 SOL
  - [ ] **Total: 10 SOL**
  
- [ ] Mainnet wallet created and funded
- [ ] Backup wallet ready
- [ ] Funds come from verified source

### Operational Preparation
- [ ] Monitoring system set up
- [ ] Alert system configured
- [ ] Logging configured
- [ ] Backup procedures documented
- [ ] Disaster recovery plan created
- [ ] Team trained on operations

## Go/No-Go Decision

### Final Checks (24 hours before launch)
- [ ] All devnet tests passed
- [ ] All documentation reviewed
- [ ] Team agrees on deployment
- [ ] Funds confirmed and ready
- [ ] Monitoring systems online
- [ ] Backup systems tested
- [ ] Emergency procedures rehearsed

### Go/No-Go Committee Sign-off
- [ ] Technical lead: _________________ Date: _______
- [ ] Operations: _________________ Date: _______
- [ ] Risk officer: _________________ Date: _______

### Final Decision
- [ ] **GO** - Proceed to mainnet
- [ ] **NO-GO** - Delay and investigate

## Mainnet Launch

### Pre-Launch (T-1 day)
- [ ] Solana configuration switched to mainnet
- [ ] Wallet has 10 SOL
- [ ] Program artifacts ready
- [ ] Services configured for mainnet
- [ ] Monitoring dashboard ready
- [ ] Incident response team on standby

### Launch (T-0)
- [ ] Deploy Brain program to mainnet
  - [ ] Program ID: ______________________________
  - [ ] Verified on https://explorer.solana.com
  
- [ ] Deploy Guardian program to mainnet
  - [ ] Program ID: ______________________________
  - [ ] Verified on explorer
  
- [ ] Deploy Agent Syndicate to mainnet
  - [ ] Program ID: ______________________________
  - [ ] Verified on explorer
  
- [ ] Deploy Fee Collector to mainnet
  - [ ] Program ID: ______________________________
  - [ ] Verified on explorer

### Pump.fun Token Creation
- [ ] Token created on Pump.fun
- [ ] Mint address: ______________________________
- [ ] Verified on Pump.fun site
- [ ] Supply: 1,000,000 BRAIN
- [ ] Launch announced

### Post-Launch (T+0)
- [ ] All 4 programs verified on explorer
- [ ] Dev buy executed successfully
- [ ] Treasury monitoring active
- [ ] Brain agent running
- [ ] Fee collection operational
- [ ] No errors in logs

### Monitoring (T+24 hours)
- [ ] Trading volume data collected
- [ ] Fee collection working
- [ ] Brain proposals generated
- [ ] Holder rewards calculated
- [ ] All systems stable
- [ ] No critical errors

### Extended Monitoring (T+1 week)
- [ ] Weekly rewards distributed
- [ ] Governance voting tested
- [ ] Partnership proposals approved
- [ ] Treasury grown from fees
- [ ] No security issues
- [ ] Community engagement metrics

## Post-Launch

### Ongoing Operations
- [ ] Daily treasury monitoring
- [ ] Weekly reward distributions
- [ ] Monthly governance reviews
- [ ] Quarterly strategy updates
- [ ] Security patches applied
- [ ] Performance optimization

### Success Metrics
- [ ] Trading volume: _____________ SOL/week
- [ ] Total fees: _____________ SOL
- [ ] Holder count: _____________
- [ ] Community engagement: _____________ %
- [ ] Program uptime: _____________ %

### Rollback Plan (If Needed)
- [ ] Circuit breaker documented
- [ ] Emergency pause procedure ready
- [ ] Recovery script tested
- [ ] Communication plan prepared

---

## Notes

```
Launch Date: _______________________________
Mainnet Program IDs:
  Brain:              _______________________________
  Guardian:           _______________________________
  Agent Syndicate:    _______________________________
  Fee Collector:      _______________________________
  
Treasury Address:   _______________________________
AI Wallet Address:  _______________________________
Token Mint:         _______________________________

Launch Notes:
_________________________________________________________________
_________________________________________________________________
_________________________________________________________________
```

---

**Last Updated**: 2024-02-05
**Status**: Ready for Devnet Testing
**Next Phase**: Devnet Integration Testing
