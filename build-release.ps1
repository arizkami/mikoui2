#!/usr/bin/env pwsh
# Convenience script to build release and copy to bin directory

$ErrorActionPreference = "Stop"

Write-Host "Building Rabital (Release)..." -ForegroundColor Cyan
Write-Host ""

# Build the project
cargo build --release --bin rabital

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "Build successful! Copying to bin directory..." -ForegroundColor Green
    
    # Run post-build script
    & .\scripts\post-build.ps1 -Profile release
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "[SUCCESS] Build complete!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Executable: .\build\release\bin\rabital.exe" -ForegroundColor Yellow
    }
} else {
    Write-Host ""
    Write-Host "[ERROR] Build failed!" -ForegroundColor Red
    exit 1
}
