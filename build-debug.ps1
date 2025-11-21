#!/usr/bin/env pwsh
# Convenience script to build debug and copy to bin directory

$ErrorActionPreference = "Stop"

Write-Host "Building Rabital (Debug)..." -ForegroundColor Cyan
Write-Host ""

# Build the project
cargo build --bin rabital

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "Build successful! Copying to bin directory..." -ForegroundColor Green
    
    # Run post-build script
    & .\scripts\post-build.ps1 -Profile debug
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "[SUCCESS] Build complete!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Executable: .\build\debug\bin\rabital.exe" -ForegroundColor Yellow
    }
} else {
    Write-Host ""
    Write-Host "[ERROR] Build failed!" -ForegroundColor Red
    exit 1
}
