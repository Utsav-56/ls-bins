#Run cargo build --release

cargo build --release
# If error occurred, exit
if ($LASTEXITCODE -ne 0) {
    Write-Host "`n`n Cargo build failed. Exiting script."
    exit $LASTEXITCODE
}

$targetBinaries = @("lsf.exe", "lsd.exe")
$targetDir = "C:\src\ls_bins"

foreach ($binaryName in $targetBinaries) {
    $destinationPath = Join-Path -Path $targetDir -ChildPath $binaryName
    $sourcePath = ".\target\release\$binaryName"
    if (Test-Path $destinationPath) {
        Remove-Item -Path $destinationPath -Force
    }

    Move-Item -Path $sourcePath -Destination $destinationPath -Force
    Write-Host "Binary moved to $targetDir successfully."
}

Clear-Host
lsf
Write-Host "`n`n`n"
lsd




