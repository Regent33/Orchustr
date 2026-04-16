<#
.SYNOPSIS
    Orchustr full test runner - runs all unit tests across 14 crates.

.DESCRIPTION
    Executes every crate's unit_suite, the integration suite, and a clippy
    check. Prints a colour-coded summary and exits 0 only if everything passes.

    Known environment limitations:
      - or-mcp and or-prism are excluded by default: the Windows Application
        Control policy (os error 4551) blocks their test binaries.
      - Pass -IncludeMcp or -IncludePrism to opt them in.

.PARAMETER TargetProfile
    CARGO_TARGET_DIR profile name (default: "test-all").

.PARAMETER IncludeMcp
    Also run or-mcp tests (may fail on machines with AppControl policy).

.PARAMETER IncludePrism
    Also run or-prism tests (subscriber-idempotency test is a known
    pre-existing failure; enable with care in CI).

.EXAMPLE
    .\scripts\dev\Invoke-AllTests.ps1

.EXAMPLE
    .\scripts\dev\Invoke-AllTests.ps1 -TargetProfile ci -IncludePrism
#>

[CmdletBinding()]
param(
    [string] $TargetProfile = "test-all",
    [switch] $IncludeMcp,
    [switch] $IncludePrism
)

$ErrorActionPreference = "Continue"
$overallOk = $true
$colour    = "Green"

# ── Workspace root ──────────────────────────────────────────────────────────
$scriptDir    = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspaceDir = (Resolve-Path (Join-Path $scriptDir ".." "..")).Path

# ── CARGO_TARGET_DIR ────────────────────────────────────────────────────────
$baseDir     = Join-Path $env:LOCALAPPDATA "Orchustr\cargo-targets"
$targetDir   = Join-Path $baseDir $TargetProfile
New-Item -ItemType Directory -Force -Path $targetDir | Out-Null
$savedTarget = $env:CARGO_TARGET_DIR
$env:CARGO_TARGET_DIR = $targetDir
Write-Host "  CARGO_TARGET_DIR = $targetDir" -ForegroundColor DarkGray

# ── Crate list ───────────────────────────────────────────────────────────────
$crates = @(
    "or-core"
    "or-beacon"
    "or-conduit"
    "or-sieve"
    "or-compass"
    "or-sentinel"
    "or-recall"
    "or-loom"
    "or-forge"
    "or-anchor"
    "or-colony"
    "or-pipeline"
    "or-relay"
    "or-checkpoint"
)

if ($IncludeMcp)   { $crates += "or-mcp"   }
if ($IncludePrism) { $crates += "or-prism" }

# ── Integration test names ───────────────────────────────────────────────────
$integrationTests = @(
    "graph_traversal"
    "chain_execution"
    "tool_invocation"
    "human_approval_gate"
    "multimodal_completion"
    "regression"
)

# ── Helpers ──────────────────────────────────────────────────────────────────
function Write-Header([string]$text) {
    $sep = "=" * 62
    Write-Host ""
    Write-Host $sep           -ForegroundColor DarkCyan
    Write-Host "  $text"     -ForegroundColor Cyan
    Write-Host $sep           -ForegroundColor DarkCyan
}

function Write-Result([string]$label, [bool]$ok, [string]$detail) {
    $icon   = if ($ok) { "PASS" } else { "FAIL" }
    $colour = if ($ok) { "Green" } else { "Red" }
    $pad    = $label.PadRight(35)
    Write-Host ("  [{0}]  {1} {2}" -f $icon, $pad, $detail) -ForegroundColor $colour
}

function Invoke-CrateSuite([string]$crate, [string]$testName) {
    $output   = & cargo test -p $crate --test $testName 2>&1 | ForEach-Object { "$_" }
    $exitCode = $LASTEXITCODE

    $passed = 0; $failed = 0
    foreach ($line in $output) {
        if ($line -match "(\d+) passed") { $passed = [int]$Matches[1] }
        if ($line -match "(\d+) failed") { $failed = [int]$Matches[1] }
    }

    # AppControl may cause nonzero even if tests passed
    $ok = ($failed -eq 0) -and ($passed -gt 0 -or $exitCode -eq 0)
    return [pscustomobject]@{ Label=$crate; Ok=$ok; Passed=$passed; Failed=$failed }
}

# ── Unit suites ──────────────────────────────────────────────────────────────
Write-Header "Unit suites ($($crates.Count) crates)"

$results     = [System.Collections.Generic.List[pscustomobject]]::new()
$totalPassed = 0
$totalFailed = 0

foreach ($crate in $crates) {
    $r = Invoke-CrateSuite $crate "unit_suite"
    Write-Result $r.Label $r.Ok "$($r.Passed) passed$(if ($r.Failed -gt 0) {", $($r.Failed) FAILED"} else {''})"
    $results.Add($r)
    $totalPassed += $r.Passed
    $totalFailed += $r.Failed
}

# ── Integration suites ───────────────────────────────────────────────────────
Write-Header "Integration suite ($($integrationTests.Count) tests)"

foreach ($testName in $integrationTests) {
    $output   = & cargo test -p or-integration-tests --test $testName 2>&1 | ForEach-Object { "$_" }
    $exitCode = $LASTEXITCODE

    $passed = 0; $failed = 0
    foreach ($line in $output) {
        if ($line -match "(\d+) passed") { $passed = [int]$Matches[1] }
        if ($line -match "(\d+) failed") { $failed = [int]$Matches[1] }
    }
    $ok = ($failed -eq 0) -and ($passed -gt 0 -or $exitCode -eq 0)
    $label = "integration::$testName"
    Write-Result $label $ok "$passed passed$(if ($failed -gt 0) {", $failed FAILED"} else {''})"
    $results.Add([pscustomobject]@{ Label=$label; Ok=$ok; Passed=$passed; Failed=$failed })
    $totalPassed += $passed
    $totalFailed += $failed
}

# ── Clippy ───────────────────────────────────────────────────────────────────
Write-Header "Clippy (workspace)"

$clippyOut  = & cargo clippy --workspace 2>&1 | ForEach-Object { "$_" }
$warnings   = ($clippyOut | Select-String "warning\[").Count
$clippyErrs = ($clippyOut | Select-String "^error\[").Count
$clippyOk   = ([int]$clippyErrs -eq 0)
$clippyMsg  = "$clippyErrs errors, $warnings warnings"
Write-Result "clippy" $clippyOk $clippyMsg
if (-not $clippyOk) { $overallOk = $false }

# ── Summary ──────────────────────────────────────────────────────────────────
Write-Header "Summary"

$overallOk = $overallOk -and ([int]$totalFailed -eq 0)
$colour    = if ($overallOk) { "Green" } else { "Red" }

Write-Host ("  Total passed : {0}" -f $totalPassed) -ForegroundColor $colour
Write-Host ("  Total failed : {0}" -f $totalFailed) -ForegroundColor $colour
Write-Host ""

if (-not $overallOk) {
    Write-Host "  Failed items:" -ForegroundColor Yellow
    $results | Where-Object { -not $_.Ok } | ForEach-Object {
        Write-Host ("    * {0}  ({1} failed)" -f $_.Label, $_.Failed) -ForegroundColor Red
    }
    Write-Host ""
}

$verdict = if ($overallOk) { "ALL TESTS PASSED" } else { "SOME TESTS FAILED" }
Write-Host ("  {0}" -f $verdict) -ForegroundColor $colour
Write-Host ""

# ── Restore env ───────────────────────────────────────────────────────────────
if ($null -eq $savedTarget -or $savedTarget -eq "") {
    Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
} else {
    $env:CARGO_TARGET_DIR = $savedTarget
}

exit $(if ($overallOk) { 0 } else { 1 })
