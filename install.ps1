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
        try { $commit = (& git -C $Source rev-parse HEAD).Trim() } catch { $commit = "unknown" }
    }

    $versionLine = Get-Content -LiteralPath (Join-Path $Source 'versions.env') |
        Where-Object { $_ -match '^TOOLKIT_VERSION=' } |
        Select-Object -First 1
    $version = if ($versionLine) {
        $versionLine -replace '^TOOLKIT_VERSION="([^"]+)"
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
if (-not (Test-Path $parent)) { New-Item -ItemType Directory -Path $parent -Force | Out-Null }

$rootResolved = (Resolve-Path $Root).Path
$destResolved = $null
if (Test-Path $PluginDest) { $destResolved = (Resolve-Path $PluginDest).Path }

if ($destResolved -and ($destResolved -eq $rootResolved)) {
    Log "checkout is already $PluginDest; refusing to copy the plugin over itself"
} elseif (Test-Path $PluginDest) {
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

function Add-UserPathEntry {
    param([string]$Entry)
    $current = [Environment]::GetEnvironmentVariable('Path', 'User')
    if ([string]::IsNullOrEmpty($current)) { $current = '' }
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
    $matcher = '^(Write|Delete)
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
    if ($doc.hooks.preToolUse) { $existing = @($doc.hooks.preToolUse) }
    $kept = @($existing | Where-Object { $_.command -ne $command })
    $entry = [pscustomobject]@{ command = $command; matcher = $matcher }
    $doc.hooks.preToolUse = @($kept + $entry)
    $json = $doc | ConvertTo-Json -Depth 6
    [System.IO.File]::WriteAllText($hooksFile, $json)
    Log "installed user preToolUse hook at $hooksFile"
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
Write-Host 'Note: Windows copies the plugin into plugins/local (no symlinks).'
Write-Host 'Re-run install.ps1 after editing the toolkit repo to refresh the copy.'
Write-Host ''
, '$1'
    } else {
        "unknown"
    }

    @($Source, $commit, $version) |
        Set-Content -LiteralPath (Join-Path $Destination '.cursor-toolkit-install') -Encoding UTF8

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

    Log "copied $Source -> $Destination"
}

$parent = Split-Path -Parent $PluginDest
if (-not (Test-Path $parent)) { New-Item -ItemType Directory -Path $parent -Force | Out-Null }

$rootResolved = (Resolve-Path $Root).Path
$destResolved = $null
if (Test-Path $PluginDest) { $destResolved = (Resolve-Path $PluginDest).Path }

if ($destResolved -and ($destResolved -eq $rootResolved)) {
    Log "checkout is already $PluginDest; refusing to copy the plugin over itself"
} elseif (Test-Path $PluginDest) {
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

function Add-UserPathEntry {
    param([string]$Entry)
    $current = [Environment]::GetEnvironmentVariable('Path', 'User')
    if ([string]::IsNullOrEmpty($current)) { $current = '' }
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
    $matcher = '^(CreatePlan|Write|StrReplace|Delete|EditNotebook)$'
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
    if ($doc.hooks.preToolUse) { $existing = @($doc.hooks.preToolUse) }
    $kept = @($existing | Where-Object { $_.command -ne $command })
    $entry = [pscustomobject]@{ command = $command; matcher = $matcher }
    $doc.hooks.preToolUse = @($kept + $entry)
    $json = $doc | ConvertTo-Json -Depth 6
    [System.IO.File]::WriteAllText($hooksFile, $json)
    Log "installed user preToolUse hook at $hooksFile"
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
Write-Host ''
Write-Host 'Note: Windows copies the plugin into plugins/local (no symlinks).'
Write-Host 'Re-run install.ps1 after editing the toolkit repo to refresh the copy.'
Write-Host ''

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
    if ($doc.hooks.preToolUse) { $existing = @($doc.hooks.preToolUse) }
    $kept = @($existing | Where-Object { $_.command -ne $command })
    $entry = [pscustomobject]@{ command = $command; matcher = $matcher }
    $doc.hooks.preToolUse = @($kept + $entry)
    $json = $doc | ConvertTo-Json -Depth 6
    [System.IO.File]::WriteAllText($hooksFile, $json)
    Log "installed user preToolUse hook at $hooksFile"
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
Write-Host ''
Write-Host 'Note: Windows copies the plugin into plugins/local (no symlinks).'
Write-Host 'Re-run install.ps1 after editing the toolkit repo to refresh the copy.'
Write-Host ''
, '$1'
    } else {
        "unknown"
    }

    @($Source, $commit, $version) |
        Set-Content -LiteralPath (Join-Path $Destination '.cursor-toolkit-install') -Encoding UTF8

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

    Log "copied $Source -> $Destination"
}

$parent = Split-Path -Parent $PluginDest
if (-not (Test-Path $parent)) { New-Item -ItemType Directory -Path $parent -Force | Out-Null }

$rootResolved = (Resolve-Path $Root).Path
$destResolved = $null
if (Test-Path $PluginDest) { $destResolved = (Resolve-Path $PluginDest).Path }

if ($destResolved -and ($destResolved -eq $rootResolved)) {
    Log "checkout is already $PluginDest; refusing to copy the plugin over itself"
} elseif (Test-Path $PluginDest) {
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

function Add-UserPathEntry {
    param([string]$Entry)
    $current = [Environment]::GetEnvironmentVariable('Path', 'User')
    if ([string]::IsNullOrEmpty($current)) { $current = '' }
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
    $matcher = '^(CreatePlan|Write|StrReplace|Delete|EditNotebook)$'
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
    if ($doc.hooks.preToolUse) { $existing = @($doc.hooks.preToolUse) }
    $kept = @($existing | Where-Object { $_.command -ne $command })
    $entry = [pscustomobject]@{ command = $command; matcher = $matcher }
    $doc.hooks.preToolUse = @($kept + $entry)
    $json = $doc | ConvertTo-Json -Depth 6
    [System.IO.File]::WriteAllText($hooksFile, $json)
    Log "installed user preToolUse hook at $hooksFile"
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
Write-Host ''
Write-Host 'Note: Windows copies the plugin into plugins/local (no symlinks).'
Write-Host 'Re-run install.ps1 after editing the toolkit repo to refresh the copy.'
Write-Host ''
