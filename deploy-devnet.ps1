#!/usr/bin/env powershell
# Brain Coin Devnet Deployment Script

Write-Host "🚀 Brain Coin - Devnet Deployment Script" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Step 1: Verify Solana Config
Write-Host "Step 1: Verifying Solana Configuration..." -ForegroundColor Green
$config = solana config get 2>&1
if ($config -match "devnet") {
    Write-Host "✅ Devnet configured" -ForegroundColor Green
} else {
    Write-Host "❌ Not on devnet, configuring..." -ForegroundColor Yellow
    solana config set --url https://api.devnet.solana.com
}

# Step 2: Check wallet
Write-Host ""
Write-Host "Step 2: Checking Wallet..." -ForegroundColor Green
$walletPath = "$env:USERPROFILE\.config\solana\id.json"
if (Test-Path $walletPath) {
    Write-Host "✅ Wallet found at $walletPath" -ForegroundColor Green
} else {
    Write-Host "❌ Wallet not found, creating new wallet..." -ForegroundColor Yellow
    solana-keygen new --no-passphrase
}

# Step 3: Show wallet address
Write-Host ""
Write-Host "Step 3: Wallet Details" -ForegroundColor Green
Write-Host "Wallet path: $walletPath" -ForegroundColor Cyan

# Step 4: Airdrop SOL (with retries)
Write-Host ""
Write-Host "Step 4: Requesting Devnet SOL..." -ForegroundColor Green
for ($i = 1; $i -le 3; $i++) {
    Write-Host "Attempt $i/3..." -ForegroundColor Yellow
    $airdrop = solana airdrop 5 2>&1
    if ($airdrop -match "Success|lamports") {
        Write-Host "✅ Airdrop successful!" -ForegroundColor Green
        break
    } else {
        if ($i -lt 3) {
            Start-Sleep -Seconds 5
        }
    }
}

# Step 5: Check balance
Write-Host ""
Write-Host "Step 5: Checking Balance..." -ForegroundColor Green
$balance = solana balance 2>&1
if ($balance -match "SOL") {
    Write-Host "✅ Balance: $balance" -ForegroundColor Green
} else {
    Write-Host "⚠️  Could not check balance, proceeding anyway..." -ForegroundColor Yellow
}

# Step 6: Verify build artifacts
Write-Host ""
Write-Host "Step 6: Checking for Compiled Programs..." -ForegroundColor Green
$buildDir = "target/debug"
$programFiles = @(
    "brain.so",
    "guardian.so", 
    "agent_syndicate.so",
    "fee_collector.so"
)

$found = @()
$missing = @()

foreach ($prog in $programFiles) {
    $path = Join-Path $buildDir $prog
    if (Test-Path $path) {
        $found += $prog
        Write-Host "✅ Found: $prog" -ForegroundColor Green
    } else {
        $missing += $prog
    }
}

if ($missing.Count -gt 0) {
    Write-Host ""
    Write-Host "⚠️  Missing programs: $($missing -join ', ')" -ForegroundColor Yellow
    Write-Host "Building programs..." -ForegroundColor Yellow
    cargo build --release
}

# Step 7: Create program keypairs
Write-Host ""
Write-Host "Step 7: Creating Program Keypairs..." -ForegroundColor Green
$programKeypairs = @{
    "brain" = "BrainProgram111111111111111111111111111111"
    "guardian" = "GuardianProgram111111111111111111111111"
    "agent-syndicate" = "SyndicProgram1111111111111111111111111"
    "fee-collector" = "FeeCollectorProgram11111111111111111111"
}

Write-Host "Using placeholder program IDs:" -ForegroundColor Cyan
foreach ($name in $programKeypairs.Keys) {
    Write-Host "  $name : $($programKeypairs[$name])" -ForegroundColor Cyan
}

# Step 8: Deploy programs
Write-Host ""
Write-Host "Step 8: Deploying Programs to Devnet..." -ForegroundColor Green
Write-Host "Note: Using simulated deployment with placeholder IDs" -ForegroundColor Yellow

$programs = @(
    @{ name = "brain"; file = "target/debug/brain.so" },
    @{ name = "guardian"; file = "target/debug/guardian.so" },
    @{ name = "agent-syndicate"; file = "target/debug/agent_syndicate.so" },
    @{ name = "fee-collector"; file = "target/debug/fee_collector.so" }
)

foreach ($prog in $programs) {
    Write-Host ""
    Write-Host "Deploying $($prog.name)..." -ForegroundColor Cyan
    if (Test-Path $prog.file) {
        Write-Host "File found: $($prog.file)" -ForegroundColor Green
        Write-Host "Ready to deploy with: solana program deploy $($prog.file)" -ForegroundColor Yellow
    } else {
        Write-Host "File not found: $($prog.file)" -ForegroundColor Red
    }
}

# Step 9: Summary
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Deployment Preparation Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next Steps:" -ForegroundColor Green
Write-Host "1. Ensure all .so files are built"
Write-Host "2. Run: cargo build --release"
Write-Host "3. Deploy each program:" -ForegroundColor Yellow
Write-Host "   solana program deploy target/release/brain.so"
Write-Host "   solana program deploy target/release/guardian.so"
Write-Host "   solana program deploy target/release/agent_syndicate.so"
Write-Host "   solana program deploy target/release/fee_collector.so"
Write-Host "4. Record the program IDs from deployment"
Write-Host "5. Update .env with program IDs"
Write-Host ""
Write-Host "Resources:" -ForegroundColor Green
Write-Host "- Devnet Explorer: https://explorer.solana.com/?cluster=devnet"
Write-Host "- Check deployment: solana program show <PROGRAM_ID>"
Write-Host "- View logs: solana logs <PROGRAM_ID>"
Write-Host ""
