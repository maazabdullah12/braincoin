#!/usr/bin/env powershell
# Brain Coin - Fix Program IDs and Deploy Script

Write-Host "🚀 Brain Coin - Devnet Deployment Script" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Use valid Solana system program as temporary IDs
$programIds = @{
    "brain" = "11111111111111111111111111111111"
    "guardian" = "11111111111111111111111111111112"
    "agent-syndicate" = "11111111111111111111111111111113"
    "fee-collector" = "11111111111111111111111111111114"
}

Write-Host "Step 1: Updating Program IDs..." -ForegroundColor Green
Write-Host "Using temporary devnet IDs:" -ForegroundColor Yellow
foreach ($name in $programIds.Keys) {
    Write-Host "  $name : $($programIds[$name])" -ForegroundColor Cyan
}
Write-Host ""

# Update brain/src/lib.rs
Write-Host "Updating brain program..." -ForegroundColor Green
$brainFile = "c:\Users\Maaz Abdullah\Documents\braincoin\programs\brain\src\lib.rs"
$brainContent = Get-Content $brainFile -Raw
$brainContent = $brainContent -replace 'declare_id!\("[^"]*"\);', "declare_id!(`"$($programIds['brain'])`");"
Set-Content -Path $brainFile -Value $brainContent
Write-Host "✅ Brain program updated" -ForegroundColor Green

# Update guardian/src/lib.rs
Write-Host "Updating guardian program..." -ForegroundColor Green
$guardianFile = "c:\Users\Maaz Abdullah\Documents\braincoin\programs\guardian\src\lib.rs"
$guardianContent = Get-Content $guardianFile -Raw
$guardianContent = $guardianContent -replace 'declare_id!\("[^"]*"\);', "declare_id!(`"$($programIds['guardian'])`");"
Set-Content -Path $guardianFile -Value $guardianContent
Write-Host "✅ Guardian program updated" -ForegroundColor Green

# Update agent-syndicate/src/lib.rs
Write-Host "Updating agent-syndicate program..." -ForegroundColor Green
$syndicateFile = "c:\Users\Maaz Abdullah\Documents\braincoin\programs\agent-syndicate\src\lib.rs"
$syndicateContent = Get-Content $syndicateFile -Raw
$syndicateContent = $syndicateContent -replace 'declare_id!\("[^"]*"\);', "declare_id!(`"$($programIds['agent-syndicate'])`");"
Set-Content -Path $syndicateFile -Value $syndicateContent
Write-Host "✅ Agent-syndicate program updated" -ForegroundColor Green

# Update fee-collector/src/lib.rs
Write-Host "Updating fee-collector program..." -ForegroundColor Green
$feeFile = "c:\Users\Maaz Abdullah\Documents\braincoin\programs\fee-collector\src\lib.rs"
$feeContent = Get-Content $feeFile -Raw
$feeContent = $feeContent -replace 'declare_id!\("[^"]*"\);', "declare_id!(`"$($programIds['fee-collector'])`");"
Set-Content -Path $feeFile -Value $feeContent
Write-Host "✅ Fee-collector program updated" -ForegroundColor Green

Write-Host ""
Write-Host "Step 2: Configuring Solana for devnet..." -ForegroundColor Green
solana config set --url https://api.devnet.solana.com
Write-Host "✅ Devnet configured" -ForegroundColor Green

Write-Host ""
Write-Host "Step 3: Building programs..." -ForegroundColor Green
cd "c:\Users\Maaz Abdullah\Documents\braincoin"
Write-Host "Running: cargo build --release" -ForegroundColor Yellow
cargo build --release

Write-Host ""
Write-Host "Step 4: Checking build results..." -ForegroundColor Green
$buildDir = "target/release"
$programs = @("brain", "guardian", "agent_syndicate", "fee_collector")

$builtCount = 0
foreach ($prog in $programs) {
    $soFile = "$buildDir\$prog.so"
    if (Test-Path $soFile) {
        $size = (Get-Item $soFile).Length / 1MB
        $sizeRounded = [math]::Round($size, 2)
        Write-Host "✅ $prog.so ($sizeRounded MB)" -ForegroundColor Green
        $builtCount++
    } else {
        Write-Host "❌ $prog.so NOT FOUND" -ForegroundColor Red
    }
}

if ($builtCount -eq 4) {
    Write-Host ""
    Write-Host "✅ All programs built successfully!" -ForegroundColor Green
    
    Write-Host ""
    Write-Host "Step 5: Ready for deployment!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Run these commands to deploy:" -ForegroundColor Yellow
    Write-Host "solana airdrop 5" -ForegroundColor Cyan
    Write-Host "solana program deploy target/release/brain.so" -ForegroundColor Cyan
    Write-Host "solana program deploy target/release/guardian.so" -ForegroundColor Cyan
    Write-Host "solana program deploy target/release/agent_syndicate.so" -ForegroundColor Cyan
    Write-Host "solana program deploy target/release/fee_collector.so" -ForegroundColor Cyan
    
} else {
    Write-Host ""
    Write-Host "❌ Build failed - $($programs.Count - $builtCount) programs missing" -ForegroundColor Red
    Write-Host "Check error messages above and fix compilation errors" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Script Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "📖 For detailed instructions, see: DEVNET_READY.md" -ForegroundColor Cyan
