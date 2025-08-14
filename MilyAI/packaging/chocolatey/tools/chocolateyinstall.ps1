$ErrorActionPreference = 'Stop'

$packageName = 'milyai'
$toolsDir   = Split-Path -Parent $MyInvocation.MyCommand.Definition
$version    = '0.1.0'

# TODO: replace with real release URL
$archiveUrl = "https://example.com/milyai/releases/$version/milyai-windows-x86_64.zip"
$archivePath = Join-Path $toolsDir 'milyai.zip'

Get-ChocolateyWebFile -PackageName $packageName -FileFullPath $archivePath -Url $archiveUrl
Get-ChocolateyUnzip -FileFullPath $archivePath -Destination $toolsDir

# Add shim
$exePath = Join-Path $toolsDir 'milyai.exe'
Install-BinFile -Name 'milyai' -Path $exePath 