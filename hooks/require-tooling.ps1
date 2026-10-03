# Blocks plan and edit tools until this conversation has called GBrain and Graphify.
# Missing transcript fails open so a hook or logging outage cannot freeze the agent.
$ErrorActionPreference = 'Stop'

function Write-HookJson {
    param([hashtable]$Payload)
    $json = $Payload | ConvertTo-Json -Compress
    [Console]::Out.WriteLine($json)
}

function Allow-Hook {
    Write-HookJson @{ permission = 'allow' }
    exit 0
}

try {
    $raw = [Console]::In.ReadToEnd()
    if ([string]::IsNullOrWhiteSpace($raw)) {
        Allow-Hook
    }

    $hookInput = $raw | ConvertFrom-Json
    $toolName = [string]$hookInput.tool_name
    $gated = $toolName -match '^(CreatePlan|Write|StrReplace|Delete|EditNotebook)$'
    if (-not $gated) {
        Allow-Hook
    }

    $transcript = [string]$hookInput.transcript_path
    if ([string]::IsNullOrWhiteSpace($transcript) -or -not (Test-Path -LiteralPath $transcript)) {
        Allow-Hook
    }

    $text = [System.IO.File]::ReadAllText($transcript)
    $hasGbrain = $text.Contains('user-gbrain')
    $hasGraphify = $text.Contains('graphify')
    if ($hasGbrain -and $hasGraphify) {
        Allow-Hook
    }

    $missing = @()
    if (-not $hasGbrain) { $missing += 'GBrain (namespace user-gbrain)' }
    if (-not $hasGraphify) { $missing += 'Graphify (graphify query, or graphify extract when graphify-out/graph.json is missing)' }

    $agent = @"
This Plan or edit is blocked until the current conversation contains both tool calls. Missing: $($missing -join '; ').
Query GBrain about the task first. If the result is empty, say so and continue from the source tree.
Run graphify query before a broad structural search. If graphify is not on PATH, use %USERPROFILE%\.local\bin\graphify.exe. If graphify-out/graph.json is missing, run graphify extract . --code-only --no-cluster first.
Read the matching Superpowers skill: writing-plans before a plan, test-driven-development before implementation, systematic-debugging for a bug, verification-before-completion before claiming the work is done.
Use Context7 when the task depends on a library, SDK, API, or CLI.
After a change, review, audit, or fix, read the matching gstack skill unless the active engineering-fix skill forbids that extra review.
Then retry this tool.
"@.Trim()

    Write-HookJson @{
        permission    = 'deny'
        user_message  = ('Plan oder ' + [char]0x00C4 + 'nderung blockiert, bis GBrain und Graphify in diesem Chat gelaufen sind.')
        agent_message = $agent
    }
    exit 0
}
catch {
    Allow-Hook
}
