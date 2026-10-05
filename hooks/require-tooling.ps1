# Best-effort compatibility guard for edit tools when a transcript path is available.
# Current Cursor's documented preToolUse payload does not guarantee transcript_path;
# the authoritative workflow anchor is SessionStart + compact always/auto-attached rules.
# Missing transcript fails open so hook/API variation cannot freeze the agent.
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
    $gated = $toolName -match '^(Write|Delete)$'
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
This edit is blocked until the current conversation contains both tool calls. Missing: $($missing -join '; ').
Query GBrain about the task first. If the result is empty, continue from repository evidence.
Run graphify query before a broad structural search; build graphify-out/graph.json first if needed.
Before editing, read the matching compact engineering rule and relevant canonical-standard sections.
Read the matching Superpowers skill; use Context7 when behavior depends on a library/SDK/API/CLI.
After material changes, use the matching gstack review/QA flow unless the active engineering-fix procedure forbids extra review.
Then retry this tool.
"@.Trim()

    Write-HookJson @{
        permission    = 'deny'
        user_message  = 'Edit blocked until GBrain and Graphify have run in this conversation.'
        agent_message = $agent
    }
    exit 0
}
catch {
    Allow-Hook
}
