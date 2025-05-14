#!/usr/bin/env pwsh
# This script is used to push changes to the remote repository
git add .

# Ask for a commit message
$commitMessage = Read-Host "Enter commit message"
git commit -m $commitMessage

# check if the remote repository is set
$remote = git remote -v | Select-String "origin" | Select-String "fetch"
if ($remote -eq $null) {
    Write-Host "No remote repository set. Please set a remote repository using 'git remote add origin <url>'"
    return
}
git push -u origin main
