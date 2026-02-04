#!/usr/bin/env pwsh

# Brain Coin Devnet Deployment Helper
# This script prepares programs for deployment to Solana devnet

Write-Host "Brain Coin Devnet Deployment Helper`n" -ForegroundColor Cyan

# 1. Check devnet configuration
Write-Host "Checking Solana Configuration..." -ForegroundColor Yellow
$config = solana config get
Write-Host $config
Write-Host ""

# 2. Check wallet
$walletPath = "$env:USERPROFILE\.config\solana\id.json"
if (Test-Path $walletPath) {
    Write-Host "Wallet found: $walletPath" -ForegroundColor Green
    $wallet = solana-keygen pubkey $walletPath
    Write-Host "  Public key: $wallet`n" -ForegroundColor Green
}
else {
    Write-Host "Wallet not found at $walletPath" -ForegroundColor Red
    Write-Host "Generate new wallet with: solana-keygen new`n" -ForegroundColor Yellow
    exit 1
}

# 3. Check balance
Write-Host "Checking wallet balance..." -ForegroundColor Yellow
$balance = solana balance -u devnet 2>&1
Write-Host $balance
Write-Host ""

# 4. Build status
Write-Host "Checking compiled programs..." -ForegroundColor Yellow
$programsDir = "target\release"

if (Test-Path $programsDir) {
    $soFiles = Get-ChildItem -Path $programsDir -Filter "*.so" -ErrorAction SilentlyContinue
    $soFiles | ForEach-Object {
        Write-Host "Found: $($_.Name)" -ForegroundColor Green
    }
    
    $dllFiles = Get-ChildItem -Path $programsDir -Filter "*.dll" -ErrorAction SilentlyContinue | Where-Object { $_.Name -notlike "*.dll.exp" -and $_.Name -notlike "*.dll.lib" }
    if ($dllFiles.Count -gt 0) {
        Write-Host "`nWindows DLL files found instead of .so files:" -ForegroundColor Yellow
        $dllFiles | ForEach-Object {
            Write-Host "  - $($_.Name)" -ForegroundColor Yellow
        }
        Write-Host "`nTo generate .so files, run:" -ForegroundColor Yellow
        Write-Host "  anchor build --skip-lint" -ForegroundColor Cyan
        Write-Host ""
    }
}
else {
    Write-Host "target\release directory not found" -ForegroundColor Red
    Write-Host "Build first with: cargo build --release or anchor build" -ForegroundColor Yellow
    exit 1
}

Write-Host "`nNEXT STEPS:" -ForegroundColor Cyan
Write-Host "1. Generate .so files: anchor build --skip-lint" -ForegroundColor White
Write-Host "2. Deploy each program:" -ForegroundColor White
Write-Host "   solana program deploy target/release/brain.so -u devnet" -ForegroundColor Gray
Write-Host "   solana program deploy target/release/guardian.so -u devnet" -ForegroundColor Gray
Write-Host "   solana program deploy target/release/agent_syndicate.so -u devnet" -ForegroundColor Gray
Write-Host "   solana program deploy target/release/fee_collector.so -u devnet" -ForegroundColor Gray
Write-Host "3. Record the program IDs output" -ForegroundColor White
Write-Host "4. Update Anchor.toml with program IDs" -ForegroundColor White
Write-Host "5. Create .env with program IDs and API keys" -ForegroundColor White
Write-Host ""
Write-Host "Use 'npm run deploy:devnet' when ready!" -ForegroundColor Green
