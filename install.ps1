[CmdletBinding()]
param(
    [string]$Version = "",
    [string]$InstallDir = ""
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Repo = "hyperb1iss/unifly"
$Binary = "unifly.exe"
$Asset = "unifly-windows-amd64.exe"

function Write-Info([string]$Message) {
    Write-Host ":: $Message" -ForegroundColor Cyan
}

function Write-Ok([string]$Message) {
    Write-Host ":: $Message" -ForegroundColor Green
}

function Write-Warn([string]$Message) {
    Write-Host ":: $Message" -ForegroundColor Yellow
}

function Get-LatestVersion {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
    if (-not $release.tag_name) {
        throw "Failed to fetch the latest release tag from GitHub."
    }

    return [string]$release.tag_name
}

function Get-InstallDir {
    if ($InstallDir) {
        return $InstallDir
    }

    return Join-Path $env:LOCALAPPDATA "unifly\bin"
}

function Ensure-PathContains([string]$PathEntry) {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $entries = @()

    if ($currentPath) {
        $entries = $currentPath.Split(';', [System.StringSplitOptions]::RemoveEmptyEntries)
    }

    if ($entries -contains $PathEntry) {
        return $false
    }

    $updatedEntries = @($entries + $PathEntry | Select-Object -Unique)
    [Environment]::SetEnvironmentVariable("Path", ($updatedEntries -join ';'), "User")
    return $true
}

function Install-Unifly([string]$ResolvedVersion, [string]$TargetDir) {
    $downloadUrl = "https://github.com/$Repo/releases/download/$ResolvedVersion/$Asset"
    $tempFile = Join-Path ([System.IO.Path]::GetTempPath()) ("unifly-" + [System.Guid]::NewGuid() + ".exe")
    $destination = Join-Path $TargetDir $Binary

    New-Item -ItemType Directory -Force -Path $TargetDir | Out-Null

    Write-Info "Downloading $Asset $ResolvedVersion..."
    Invoke-WebRequest -Uri $downloadUrl -OutFile $tempFile

    Write-Info "Installing to $destination..."
    Move-Item -Force $tempFile $destination

    return $destination
}

Write-Host ""
Write-Host "  unifly installer" -ForegroundColor Magenta
Write-Host ""

$resolvedVersion = if ($Version) { $Version } else { Get-LatestVersion }
$resolvedInstallDir = Get-InstallDir
$binaryPath = Install-Unifly -ResolvedVersion $resolvedVersion -TargetDir $resolvedInstallDir
$pathUpdated = Ensure-PathContains -PathEntry $resolvedInstallDir
$installedVersion = & $binaryPath --version

Write-Ok "Installed $installedVersion"

if ($pathUpdated) {
    Write-Warn "Added $resolvedInstallDir to your user PATH."
    Write-Warn "Open a new terminal before running unifly from anywhere."
}

Write-Host ""
Write-Host "  Get started:" -ForegroundColor Cyan
Write-Host "    unifly config init    # Set up your controller"
Write-Host "    unifly devices list   # List network devices"
Write-Host "    unifly tui            # Launch the dashboard"
Write-Host ""
