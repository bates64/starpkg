New-Item -ItemType Directory -Force -Path "$HOME\.starpkg"

Invoke-WebRequest -Uri "https://github.com/nanaian/starpkg/releases/latest/download/starpkg-x86_64-pc-windows-msvc.zip" -OutFile "~\Downloads\starpkg.zip"
Expand-Archive -Force -Path "~\Downloads\starpkg.zip" -DestinationPath "$HOME\.starpkg"
Remove-Item "~\Downloads\starpkg.zip"

$path = [Environment]::GetEnvironmentVariable("Path", "User")
if ($path -notlike "*$HOME\.starpkg*") {
    [Environment]::SetEnvironmentVariable("Path", "$HOME\.starpkg;" + $path, "User")
}

Write-Output "starpkg installed/updated and added to PATH"
