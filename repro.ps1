# PowerShell reproduction script
Write-Host "Cleaning shared target directory..."
if (Test-Path "shared-target") {
    Remove-Item -Path "shared-target" -Recurse -Force
}

Write-Host "Starting builds..."

$p1 = Start-Process cargo -ArgumentList "build", "--manifest-path", "package-a/Cargo.toml", "--target-dir", "shared-target" -PassThru -NoNewWindow
$p2 = Start-Process cargo -ArgumentList "build", "--manifest-path", "package-b/Cargo.toml", "--target-dir", "shared-target" -PassThru -NoNewWindow

$p1 | Wait-Process
$p2 | Wait-Process

Write-Host "Builds finished."
