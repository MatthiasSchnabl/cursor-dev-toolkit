#Requires -Version 5.1
$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
$PluginDest = Join-Path $env:USERPROFILE ".cursor\plugins\local\cursor-dev-toolkit"
$ExcludeDirs = @('.git', '.cursor-toolkit-install')

function Log($msg) { Write-Host "install: $msg" }

function Write-InstallMarker {
    param(
        [string]$Source,
        [string]$Destination
    )

    $commit = "unknown"
    $git = Get-Command git -ErrorAction SilentlyContinue
    if ($git) {
        try {
            $commit = (& git -C $Source rev-parse HEAD).Trim()
        } catch {
            $commit = "unknown"
        }
    }

    $versionLine = Get-Content -LiteralPath (Join-Path $Source 'versions.env') |
        Where-Object { $_ -match '^TOOLKIT_VERSION=' } |
        Select-Object -First 1
    $version = if ($versionLine) {
        $versionLine -replace '^TOOLKIT_VERSION="([^"]+)"$', '$1'
    } else {
        "unknown"
    }

    $marker = Join-Path $Destination '.cursor-toolkit-install'
    $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllLines($marker, @($Source, $commit, $version), $utf8NoBom)
    Log "wrote install provenance marker"
}

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

    Write-InstallMarker -Source $Source -Destination $Destination
    Log "copied $Source -> $Destination"
}

$parent = Split-Path -Parent $PluginDest
if (-not (Test-Path $parent)) {
    New-Item -ItemType Directory -Path $parent -Force | Out-Null
}

$rootResolved = (Resolve-Path $Root).Path
$destResolved = $null
if (Test-Path $PluginDest) {
    $destResolved = (Resolve-Path $PluginDest).Path
}

if (Test-Path $PluginDest) {
    $item = Get-Item $PluginDest -Force
    if ($item.LinkType -in @('SymbolicLink', 'Junction')) {
        Log "replacing external link with a real local plugin copy"
        Remove-Item $PluginDest -Force -Recurse
        Copy-PluginTree -Source $Root -Destination $PluginDest
    } elseif ($destResolved -and ($destResolved -eq $rootResolved)) {
        Log "checkout is already $PluginDest; no plugin copy required"
    } else {
        $manifest = Join-Path $PluginDest ".cursor-plugin\plugin.json"
        if (-not (Test-Path $manifest)) {
            throw "refusing to overwrite unrelated plugin directory: $PluginDest"
        }
        Log "refreshing plugin copy at $PluginDest"
        Copy-PluginTree -Source $Root -Destination $PluginDest
    }
} else {
    Copy-PluginTree -Source $Root -Destination $PluginDest
}

function Add-UserPathEntry {
    param([string]$Entry)

    $current = [Environment]::GetEnvironmentVariable('Path', 'User')
    if ([string]::IsNullOrEmpty($current)) {
        $current = ''
    }

    $present = $current.Split(';') | Where-Object { $_.TrimEnd('\') -eq $Entry.TrimEnd('\') }
    if ($present) {
        Log "user PATH already contains $Entry"
        return
    }

    $updated = if ([string]::IsNullOrEmpty($current)) { $Entry } else { "$Entry;$current" }
    [Environment]::SetEnvironmentVariable('Path', $updated, 'User')
    Log "prepended $Entry to the user PATH"
}

function Install-UserToolingHook {
    $cursorHome = Join-Path $env:USERPROFILE '.cursor'
    $hooksDir = Join-Path $cursorHome 'hooks'
    New-Item -ItemType Directory -Path $hooksDir -Force | Out-Null
    Copy-Item -LiteralPath (Join-Path $Root 'hooks\require-tooling.ps1') -Destination (Join-Path $hooksDir 'require-tooling.ps1') -Force

    $hooksFile = Join-Path $cursorHome 'hooks.json'
    $command = 'powershell -NoProfile -ExecutionPolicy Bypass -File ./hooks/require-tooling.ps1'
    $matcher = '^(Write|Delete)$'

    $doc = $null
    if (Test-Path $hooksFile) {
        $doc = Get-Content -LiteralPath $hooksFile -Raw | ConvertFrom-Json
    }

    if (-not $doc) {
        $doc = [pscustomobject]@{ version = 1; hooks = [pscustomobject]@{} }
    }

    if (-not $doc.hooks) {
        $doc | Add-Member -NotePropertyName hooks -NotePropertyValue ([pscustomobject]@{}) -Force
    }

    $existing = @()
    if ($doc.hooks.PSObject.Properties.Name -contains 'preToolUse') {
        $existing = @($doc.hooks.preToolUse)
    }

    $kept = @($existing | Where-Object { $_.command -ne $command })
    $entry = [pscustomobject]@{ command = $command; matcher = $matcher }
    $updatedHooks = @($kept + $entry)

    if ($doc.hooks.PSObject.Properties.Name -contains 'preToolUse') {
        $doc.hooks.preToolUse = $updatedHooks
    } else {
        $doc.hooks | Add-Member -NotePropertyName preToolUse -NotePropertyValue $updatedHooks
    }

    $json = $doc | ConvertTo-Json -Depth 6
    [System.IO.File]::WriteAllText($hooksFile, $json)
    Log "installed best-effort user preToolUse hook at $hooksFile"
}

Add-UserPathEntry (Join-Path $env:USERPROFILE '.local\bin')
Install-UserToolingHook

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
Write-Host '  4. Run /engineering-context-doctor if rules/skills look stale'
Write-Host ''
Write-Host 'Windows copies the plugin into plugins/local when the source checkout lives elsewhere.'
Write-Host 'Re-run install.ps1 after editing the toolkit source checkout to refresh the installed copy.'
Write-Host ''
