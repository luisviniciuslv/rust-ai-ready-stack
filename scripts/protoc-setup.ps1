param(
    [string]$ProtocVersion = "29.3"
)

$ErrorActionPreference = "Stop"

$toolsDir = ".tools/protoc"
$zipName = "protoc-$ProtocVersion-win64.zip"
$url = "https://github.com/protocolbuffers/protobuf/releases/download/v$ProtocVersion/$zipName"
$zipPath = Join-Path $toolsDir $zipName
$protocBin = ".tools/protoc/bin/protoc.exe"

New-Item -ItemType Directory -Force -Path $toolsDir | Out-Null

Write-Host "[protoc] Downloading $url"
Invoke-WebRequest -Uri $url -OutFile $zipPath

Write-Host "[protoc] Extracting $zipPath"
Expand-Archive -Force $zipPath $toolsDir

New-Item -ItemType Directory -Force -Path ".cargo" | Out-Null

$config = @"
[env]
PROTOC = { value = "$protocBin", relative = true }
"@

Set-Content -Encoding UTF8 ".cargo/config.toml" $config

Write-Host "[protoc] Configured: $protocBin"
Write-Host "[protoc] Cargo env written to .cargo/config.toml"
