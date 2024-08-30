$ErrorActionPreference = "Stop"

# Determine architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "amd64" } else { "386" }

# Set installation directory
$installDir = "$env:USERPROFILE\.crabodex"

# Create installation directory if it doesn't exist
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

# Determine latest release
$latestRelease = (Invoke-RestMethod "https://api.github.com/repos/fabien-h/crabodex/releases/latest").tag_name

# Download binary
$binaryName = "crabodex-windows-$arch.exe"
$downloadUrl = "https://github.com/fabien-h/crabodex/releases/download/$latestRelease/$binaryName"

Write-Host "Downloading Crabodex from $downloadUrl"
Invoke-WebRequest -Uri $downloadUrl -OutFile "$installDir\crabodex.exe"

# Add to PATH if not already there
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$installDir", "User")
    Write-Host "Crabodex has been added to your PATH. Please restart your terminal to use the 'crabodex' command."
}

Write-Host "Crabodex has been installed to $installDir\crabodex.exe"
Write-Host "You can now use the 'crabodex' command."