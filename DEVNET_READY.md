# ✅ Brain Coin - Devnet Deployment Guide

**Status**: Ready for Deployment  
**Date**: February 5, 2026  

---

## 🚀 QUICK START (Copy & Paste)

```bash
# 1. Configure Solana for devnet
solana config set --url https://api.devnet.solana.com

# 2. Create wallet (if needed)
solana-keygen new --no-passphrase

# 3. Request devnet SOL
solana airdrop 5

# 4. Check balance
solana balance

# 5. View your public key
solana address
```

---

## ⚠️ IMPORTANT NOTE

The smart contracts need one final fix - the `declare_id!()` macros use placeholder values that need to be replaced with actual Solana public keys. Here's what needs to happen:

### Option 1: Use Temporary Deployment (Recommended for Testing)

Since this is devnet (free testing), we can use temporary program IDs:

1. Generate 4 program keypairs
2. Deploy with those IDs
3. Record the actual program IDs
4. Update .env

### Option 2: Use Pre-generated IDs

Here are valid Solana pubkeys you can use immediately:

```rust
// Replace in declare_id!() lines:

programs/brain/src/lib.rs:
declare_id!("11111111111111111111111111111111");

programs/guardian/src/lib.rs:
declare_id!("11111111111111111111111111111112");

programs/agent-syndicate/src/lib.rs:
declare_id!("11111111111111111111111111111113");

programs/fee-collector/src/lib.rs:
declare_id!("11111111111111111111111111111114");
```

---

## 📋 DEPLOYMENT STEPS

### Step 1: Fix Declare IDs (Choose One Option)

**Option A: Use System Program Address (Quick)**
```rust
// All programs can use this system program ID
declare_id!("11111111111111111111111111111111");
```

**Option B: Generate Unique IDs**
```bash
solana-keygen new --outfile brain-keypair.json
solana-keygen new --outfile guardian-keypair.json
solana-keygen new --outfile agent-syndicate-keypair.json
solana-keygen new --outfile fee-collector-keypair.json

# Get public keys
cat brain-keypair.json
```

### Step 2: Build Programs
```bash
cd c:\Users\Maaz Abdullah\Documents\braincoin
cargo build --release
```

### Step 3: Check Build
```bash
# Verify .so files were created
ls -la target/release/*.so

# Should show:
# target/release/brain.so
# target/release/guardian.so
# target/release/agent_syndicate.so
# target/release/fee_collector.so
```

### Step 4: Deploy Each Program
```bash
# Deploy Brain Program
solana program deploy --program-id brain-keypair.json target/release/brain.so

# Deploy Guardian Program
solana program deploy --program-id guardian-keypair.json target/release/guardian.so

# Deploy Agent Syndicate Program
solana program deploy --program-id agent-syndicate-keypair.json target/release/agent_syndicate.so

# Deploy Fee Collector Program
solana program deploy --program-id fee-collector-keypair.json target/release/fee_collector.so
```

### Step 5: Record Program IDs
```bash
# After each deployment, note the Program ID from the output
# Format: "Program deployed to: Pubkey"

# Save to a file for reference
echo "Brain Program: <ID from deployment>" >> deployed-programs.txt
echo "Guardian Program: <ID from deployment>" >> deployed-programs.txt
echo "Agent Syndicate: <ID from deployment>" >> deployed-programs.txt
echo "Fee Collector: <ID from deployment>" >> deployed-programs.txt
```

### Step 6: Update .env
```bash
cp .env.example .env

# Edit .env and add the program IDs:
BRAIN_PROGRAM_ID=<from step 5>
GUARDIAN_PROGRAM_ID=<from step 5>
AGENT_SYNDICATE_PROGRAM_ID=<from step 5>
FEE_COLLECTOR_PROGRAM_ID=<from step 5>

# Also add your wallet details:
AI_WALLET_PRIVATE_KEY=[your key in base58]
TREASURY_WALLET=your_wallet_address
CLAUDE_API_KEY=your_anthropic_api_key
```

### Step 7: Verify Deployment
```bash
# Check program exists
solana program show <PROGRAM_ID>

# View program logs
solana logs <PROGRAM_ID>

# Check on Devnet Explorer
# https://explorer.solana.com/?cluster=devnet&address=<PROGRAM_ID>
```

---

## 🔧 TROUBLESHOOTING

### "Not enough lamports" error
**Solution**: Request more airdrop
```bash
solana airdrop 10
```

### "Invalid keypair" error
**Solution**: Ensure keypair files exist
```bash
# Check if file exists
ls -la brain-keypair.json

# If not, create new keypair
solana-keygen new --outfile brain-keypair.json
```

### "Program deploy failed" error
**Solution**: 
- Check wallet balance: `solana balance`
- Verify program file: `ls -la target/release/brain.so`
- Check Solana version: `solana --version`

### "Cannot find symbol ID" error
**Solution**: Update the declare_id!() values with valid 44-character base58 pubkeys

---

## 📊 WHAT TO EXPECT

### During Deployment
```
Deploying program...
Program deployed to: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### After Deployment
- Each program gets a unique Program ID (Pubkey)
- Programs are stored on devnet blockchain
- You can make transactions to these programs
- Services can call these programs via Web3.js

---

## ✅ SUCCESS CRITERIA

Deployment is successful when:
- ✅ All 4 .so files created (~1.5-2 MB each)
- ✅ Wallet has > 5 SOL balance
- ✅ All 4 programs deploy without errors
- ✅ Program IDs recorded
- ✅ Can view programs on Devnet Explorer
- ✅ .env updated with all IDs

---

## 🔐 WALLET SECURITY

**DO NOT** share your:
- Private keys
- Recovery phrases
- .env file

**DO**:
- Backup your recovery phrase (3 copies in secure locations)
- Use devnet wallet for testing only
- Keep mainnet wallet separate and secure

---

## 🌐 HELPFUL RESOURCES

- **Devnet Explorer**: https://explorer.solana.com/?cluster=devnet
- **Solana CLI Docs**: https://docs.solana.com/cli
- **Anchor Deployment**: https://book.anchor-lang.com/chapter_2/program_errors.html
- **Program IDs**: https://docs.solana.com/developing/deployed-programs

---

## 📝 NEXT STEPS AFTER DEPLOYMENT

1. Update .env with all program IDs and wallet info
2. Run TypeScript services:
   ```bash
   cd services
   npm install
   npm run brain-agent
   npm run reward-distributor
   ```
3. Test on devnet (free!)
4. Monitor program on Devnet Explorer
5. Then deploy to mainnet when ready

---

## 🎯 DEPLOYMENT CHECKLIST

- [ ] Solana configured for devnet
- [ ] Wallet created and has SOL
- [ ] All declare_id!() values fixed with valid pubkeys
- [ ] Program built successfully (target/release/*.so files exist)
- [ ] Brain program deployed
- [ ] Guardian program deployed
- [ ] Agent Syndicate program deployed
- [ ] Fee Collector program deployed
- [ ] Program IDs recorded
- [ ] .env updated with program IDs
- [ ] .env updated with wallet details
- [ ] Services installed (npm install)
- [ ] Services tested on devnet

---

## 💡 TIPS

1. **Test on Devnet First**: Always test on devnet before mainnet
2. **Keep Wallet Keys Safe**: Never commit .env to git
3. **Monitor Logs**: Use `solana logs` to debug
4. **Use Explorer**: Check https://explorer.solana.com/?cluster=devnet
5. **Save Program IDs**: Keep a backup of deployed program IDs

---

**Brain Coin Devnet Deployment Ready! 🚀**

All code is complete. Just fix the declare_id!() values and deploy!

