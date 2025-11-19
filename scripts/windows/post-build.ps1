# Post-build script to organize output in CMake style
# Usage: .\post-build.ps1 [debug|release]

param(
    [string]$Config = "release"
)

$BuildDir = "build\$Config"
$BinDir = "$BuildDir\bin"
$SharedDir = "$BuildDir\shared"

Write-Host "Organizing build output for $Config configuration..." -ForegroundColor Cyan

# Create bin directory
if (-not (Test-Path $BinDir)) {
    New-Item -ItemType Directory -Path $BinDir -Force | Out-Null
}

# Move executable to bin directory
$ExeName = "rabital.exe"
$SourceExe = "$BuildDir\$ExeName"
$DestExe = "$BinDir\$ExeName"

if (Test-Path $SourceExe) {
    Write-Host "Moving $ExeName to bin directory..." -ForegroundColor Green
    Move-Item -Path $SourceExe -Destination $DestExe -Force
    
    # Also move .pdb file if it exists
    $SourcePdb = "$BuildDir\rabital.pdb"
    if (Test-Path $SourcePdb) {
        Move-Item -Path $SourcePdb -Destination "$BinDir\rabital.pdb" -Force
    }
    
    # Move .d file if it exists
    $SourceD = "$BuildDir\rabital.d"
    if (Test-Path $SourceD) {
        Move-Item -Path $SourceD -Destination "$BinDir\rabital.d" -Force
    }
} else {
    Write-Host "Warning: $ExeName not found at $SourceExe" -ForegroundColor Yellow
}

# Verify shared folder was copied
if (Test-Path $SharedDir) {
    Write-Host "Shared folder found at $SharedDir" -ForegroundColor Green
} else {
    Write-Host "Warning: Shared folder not found at $SharedDir" -ForegroundColor Yellow
}

Write-Host "`nBuild organization complete!" -ForegroundColor Cyan
Write-Host "Output structure:" -ForegroundColor White
Write-Host "  $BinDir\" -ForegroundColor Gray
Write-Host "    - rabital.exe" -ForegroundColor Gray
Write-Host "  $SharedDir\" -ForegroundColor Gray
Write-Host "    - config/" -ForegroundColor Gray
Write-Host "    - themes/" -ForegroundColor Gray
