$ErrorActionPreference = "Stop"

if (Test-Path ".tools/protoc") {
    Remove-Item -Recurse -Force ".tools/protoc"
    Write-Host "[protoc] Removed .tools/protoc"
}

New-Item -ItemType Directory -Force -Path ".cargo" | Out-Null

$config = @"
[alias]
protoc-setup = "run --quiet --bin xtask -- protoc-setup"
protoc-clean = "run --quiet --bin xtask -- protoc-clean"
"@

Set-Content -Encoding UTF8 ".cargo/config.toml" $config
Write-Host "[protoc] Reset .cargo/config.toml (aliases preserved)"
