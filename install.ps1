#Requires -Version 5.1
$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
$PluginDest = Join-Path $env:USERPROFILE ".cursor\plugins\local\cursor-dev-toolkit"

function Log($msg) { Write-Host "install: $msg" }

$parent = Split-Path -Parent $PluginDest
if (-not (Test-Path $parent)) { New-Item -ItemType Directory -Path $parent -Force | Out-Null }

if (Test-Path $PluginDest) {
    $rootResolved = (Resolve-Path $Root).Path
    $destResolved = (Resolve-Path $PluginDest).Path
    if ($destResolved -eq $rootResolved) {
        Log "plugin already installed at $PluginDest"
    } else {
        $item = Get-Item $PluginDest -Force
        if ($item.LinkType -in @('SymbolicLink', 'Junction')) {
            Remove-Item $PluginDest -Force -Recurse
        } else {
            throw "refusing to overwrite existing $PluginDest"
        }
    }
}

if (-not (Test-Path $PluginDest)) {
    try {
        New-Item -ItemType SymbolicLink -Path $PluginDest -Target $Root -Force | Out-Null
        Log "linked $PluginDest -> $Root"
    } catch {
        Log "symlink failed ($($_.Exception.Message)); trying directory junction..."
        cmd /c mklink /J "$PluginDest" "$Root" | Out-Null
        if (-not (Test-Path $PluginDest)) {
            Log "junction failed; copying plugin tree..."
            Copy-Item -Path $Root -Destination $PluginDest -Recurse -Force
            Log "copied $Root -> $PluginDest"
        } else {
            Log "junctioned $PluginDest -> $Root"
        }
    }
}

function To-BashPath([string]$Path) {
    $resolved = (Resolve-Path $Path).Path
    if ($resolved -match '^([A-Za-z]):\\(.*)$') {
        $drive = $Matches[1].ToLower()
        $rest = $Matches[2] -replace '\\','/'
        $bashCmd = Get-Command bash -ErrorAction SilentlyContinue
        if ($bashCmd -and $bashCmd.Source -like '*system32*') {
            return "/mnt/$drive/$rest"
        }
        return "/$drive/$rest"
    }
    return $resolved -replace '\\','/'
}

$bash = Get-Command bash -ErrorAction SilentlyContinue
if ($bash) {
    Log "running bootstrap via Git Bash..."
    $bootstrap = To-BashPath (Join-Path $Root "scripts\bootstrap.sh")
    & bash $bootstrap
} else {
    Log 'bash not found - run scripts/bootstrap.sh from Git Bash or WSL'
}

Write-Host ''
Write-Host 'cursor-dev-toolkit installed.'
Write-Host ''
Write-Host 'Next steps:'
Write-Host '  1. Cursor -> Developer: Reload Window'
Write-Host '  2. Customize -> confirm cursor-dev-toolkit under User scope'
Write-Host '  3. Run scripts/verify.sh via Git Bash'
Write-Host ''
