$ErrorActionPreference = "Stop"

if (Test-Path ".tools/protoc") {
    Remove-Item -Recurse -Force ".tools/protoc"
    Write-Host "[protoc] Removed .tools/protoc"
}

if (Test-Path ".cargo/config.toml") {
    Remove-Item -Force ".cargo/config.toml"
    Write-Host "[protoc] Removed .cargo/config.toml"
}
