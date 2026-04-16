param(
    [string]$TargetProfile = "windows-local",
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CargoArgs
)

if (-not $CargoArgs -or $CargoArgs.Count -eq 0) {
    throw "Pass the cargo arguments after the profile name, for example: .\\scripts\\dev\\Invoke-CargoExternalTarget.ps1 phase6 test -p or-prism"
}

$baseDir = Join-Path $env:LOCALAPPDATA "Orchustr\\cargo-targets"
$targetDir = Join-Path $baseDir $TargetProfile
$previousTargetDir = $env:CARGO_TARGET_DIR

New-Item -ItemType Directory -Force -Path $targetDir | Out-Null

try {
    $env:CARGO_TARGET_DIR = $targetDir
    Write-Host "Using CARGO_TARGET_DIR=$targetDir"
    & cargo @CargoArgs
    $exitCode = $LASTEXITCODE
} finally {
    if ($null -eq $previousTargetDir -or $previousTargetDir -eq "") {
        Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
    } else {
        $env:CARGO_TARGET_DIR = $previousTargetDir
    }
}

exit $exitCode
