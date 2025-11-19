# Build script for Rabital
# Usage: .\build-app.ps1 [debug|release]

param(
    [string]$Config = "release"
)

Write-Host "Building Rabital ($Config)..." -ForegroundColor Cyan
Write-Host ""

# Build the project
if ($Config -eq "release") {
    cargo build --release --bin rabital
} else {
    cargo build --bin rabital
}

if ($LASTEXITCODE -eq 0) {
    Write-Host "`nBuild successful!" -ForegroundColor Green
    Write-Host ""
    
    # Run post-build organization
    & .\post-build.ps1 -Config $Config
    
    Write-Host "`nTo run the application:" -ForegroundColor Cyan
    Write-Host "  .\build\$Config\bin\rabital.exe" -ForegroundColor White
} else {
    Write-Host "`nBuild failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}
