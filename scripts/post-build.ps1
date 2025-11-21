# Post-build script to copy executable to bin directory
param(
    [string]$Profile = "release"
)

$ErrorActionPreference = "Stop"

$buildDir = "build\$Profile"
$binDir = "$buildDir\bin"
$exeName = "rabital.exe"

# Create bin directory if it doesn't exist
if (-not (Test-Path $binDir)) {
    New-Item -ItemType Directory -Path $binDir -Force | Out-Null
    Write-Host "Created directory: $binDir"
}

# Copy executable
$source = "$buildDir\$exeName"
$destination = "$binDir\$exeName"

if (Test-Path $source) {
    Copy-Item -Path $source -Destination $destination -Force
    Write-Host "Copied $exeName to $binDir"
    Write-Host "Executable location: $destination"
} else {
    Write-Error "Executable not found at: $source"
    exit 1
}

# Copy PDB file if it exists (for debugging)
$pdbSource = "$buildDir\rabital.pdb"
$pdbDest = "$binDir\rabital.pdb"
if (Test-Path $pdbSource) {
    Copy-Item -Path $pdbSource -Destination $pdbDest -Force
    Write-Host "Copied rabital.pdb to $binDir"
}

Write-Host "`nBuild complete! Run the application with:"
Write-Host "  .\$destination"
