#Requires -Version 5.1
$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
$PluginDest = Join-Path $env:USERPROFILE ".cursor\plugins\local\cursor-dev-toolkit"
$ExcludeDirs = @('.git')

function Log($msg) { Write-Host "install: $msg" }

function Copy-PluginTree {
    param(
        [string]$Source,
        [string]$Destination
    )

    if (Test-Path $Destination) {
        Remove-Item $Destination -Recurse -Force
    }

    New-Item -ItemType Directory -Path $Destination -Force | Out-Null

    $excludeArgs = $ExcludeDirs | ForEach-Object { "/XD", $_ }
    & robocopy $Source $Destination /MIR /NFL /NDL /NJH /NJS /NC /NS @excludeArgs | Out-Null
    if ($LASTEXITCODE -ge 8) {
        throw "failed to copy plugin tree (robocopy exit $LASTEXITCODE)"
    }

    Log "copied $Source -> $Destination"
}

$parent = Split-Path -Parent $PluginDest
if (-not (Test-Path $parent)) { New-Item -ItemType Directory -Path $parent -Force | Out-Null }

if (Test-Path $PluginDest) {
    $item = Get-Item $PluginDest -Force
    if ($item.LinkType -in @('SymbolicLink', 'Junction')) {
        Log "removing external link (Cursor rejects plugins outside plugins/local)"
        Remove-Item $PluginDest -Force -Recurse
    } else {
        $manifest = Join-Path $PluginDest ".cursor-plugin\plugin.json"
        if (Test-Path $manifest) {
            Log "refreshing plugin copy at $PluginDest"
        }
        Copy-PluginTree -Source $Root -Destination $PluginDest
    }
}

if (-not (Test-Path $PluginDest)) {
    Copy-PluginTree -Source $Root -Destination $PluginDest
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
Write-Host 'Note: Windows copies the plugin into plugins/local (no symlinks).'
Write-Host 'Re-run install.ps1 after editing the toolkit repo to refresh the copy.'
Write-Host ''
