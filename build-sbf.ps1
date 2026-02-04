#!/usr/bin/env pwsh

# Brain Coin - Automated Build Script for SBF Programs
# Attempts multiple methods to generate .so files for Solana deployment

$ErrorActionPreference = "Continue"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Brain Coin SBF Build Automation" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

function Test-Command {
    param($Command)
    try {
        & $Command --version | Out-Null
        return $true
    }
    catch {
        return $false
    }
}

# Check for required tools
Write-Host "Checking build tools..." -ForegroundColor Yellow
$tools = @{
    "cargo" = "cargo"
    "rustup" = "rustup"
    "solana" = "solana"
}

foreach ($tool in $tools.Keys) {
    if (Test-Command $tools[$tool]) {
        Write-Host "  [OK] $tool" -ForegroundColor Green
    }
    else {
        Write-Host "  [MISSING] $tool" -ForegroundColor Red
    }
}

Write-Host ""

# Method 1: Try to install and use cargo-build-sbf
Write-Host "Attempting Method 1: cargo-build-sbf..." -ForegroundColor Yellow
Write-Host "Installing SBF toolchain..." -ForegroundColor Cyan

$sbfInstalled = $false
try {
    rustup target add sbf-solana-solana 2>&1 | Out-Null
    Write-Host "[OK] SBF target added`n" -ForegroundColor Green
    
    # Try building with cargo-build-sbf
    if (Test-Path "programs/brain/Cargo.toml") {
        Write-Host "Building brain program..." -ForegroundColor Cyan
        cargo build-sbf --manifest-path "programs/brain/Cargo.toml" --release 2>&1
        
        if ($LASTEXITCODE -eq 0) {
            $sbfInstalled = $true
            Write-Host "[SUCCESS] cargo-build-sbf working!" -ForegroundColor Green
        }
    }
}
catch {
    Write-Host "[FAILED] SBF toolchain setup: $_" -ForegroundColor Red
}

if (!$sbfInstalled) {
    Write-Host "`nMethod 1 did not complete. This is normal on Windows.`n" -ForegroundColor Yellow
    
    Write-Host "========================================" -ForegroundColor Yellow
    Write-Host "Recommended Solutions" -ForegroundColor Yellow
    Write-Host "========================================`n" -ForegroundColor Yellow
    
    Write-Host "1. Install Solana Development Environment" -ForegroundColor Cyan
    Write-Host "   Run in WSL (Windows Subsystem for Linux):" -ForegroundColor Gray
    Write-Host "   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" -ForegroundColor DarkGray
    Write-Host "   $ cargo install cargo-build-sbf" -ForegroundColor DarkGray
    Write-Host ""
    
    Write-Host "2. Use Docker Container" -ForegroundColor Cyan
    Write-Host "   Use official Solana Docker image:" -ForegroundColor Gray
    Write-Host "   $ docker run -v ${PWD}:/app solanalabs/rust:latest cargo build-sbf ..." -ForegroundColor DarkGray
    Write-Host ""
    
    Write-Host "3. Use online compilation service" -ForegroundColor Cyan
    Write-Host "   Upload code to: https://beta.solpg.io/" -ForegroundColor Gray
    Write-Host ""
    
    Write-Host "4. Cross-compile on Linux/Mac" -ForegroundColor Cyan
    Write-Host "   Clone this repo on a Linux/macOS machine" -ForegroundColor Gray
    Write-Host "   Run: cargo build-sbf --manifest-path programs/*/Cargo.toml --release" -ForegroundColor DarkGray
    Write-Host "   Copy generated .so files back to this machine" -ForegroundColor DarkGray
    Write-Host ""
}
else {
    Write-Host ""
    Write-Host "Successfully generated .so files!" -ForegroundColor Green
    Write-Host "Next: Verify files exist in target/sbf-solana-solana/release/" -ForegroundColor Green
}

Write-Host ""
Write-Host "Current Status:" -ForegroundColor Yellow
Write-Host ""

# Check what we have
$soFiles = Get-ChildItem -Path "target" -Filter "*.so" -Recurse -ErrorAction SilentlyContinue
$dllFiles = Get-ChildItem -Path "target/release" -Filter "*.dll" -ErrorAction SilentlyContinue | Where-Object { $_.Name -notlike "*.dll.exp" -and $_.Name -notlike "*.dll.lib" }

if ($soFiles.Count -gt 0) {
    Write-Host "SO Files Found:" -ForegroundColor Green
    $soFiles | ForEach-Object {
        Write-Host "  [OK] $($_.FullName)" -ForegroundColor Green
    }
}
else {
    Write-Host "SO Files: NONE FOUND (still needed)" -ForegroundColor Red
}

Write-Host ""

if ($dllFiles.Count -gt 0) {
    Write-Host "Windows DLL Files Found (Windows artifacts):" -ForegroundColor Yellow
    $dllFiles | ForEach-Object {
        Write-Host "  [INFO] $($_.Name)" -ForegroundColor Gray
    }
    Write-Host ""
    Write-Host "Note: DLL files cannot be deployed to Solana. Need .so files." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "For immediate deployment testing:" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan
Write-Host "Use WSL or Docker to generate .so files, then:" -ForegroundColor White
Write-Host "$ solana program deploy target/sbf-solana-solana/release/brain.so -u devnet" -ForegroundColor Gray
Write-Host "$ solana program deploy target/sbf-solana-solana/release/guardian.so -u devnet" -ForegroundColor Gray
Write-Host "$ solana program deploy target/sbf-solana-solana/release/agent_syndicate.so -u devnet" -ForegroundColor Gray
Write-Host "$ solana program deploy target/sbf-solana-solana/release/fee_collector.so -u devnet" -ForegroundColor Gray
