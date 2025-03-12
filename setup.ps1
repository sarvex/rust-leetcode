# Remove all files except JS and TS files
Get-ChildItem -Path . -Recurse -File | Where-Object { $_.Extension -notin ".rs" } | Remove-Item -Force

# Function to process code files
function Process-Files($extension) {
    Get-ChildItem -Path . -Recurse -Filter "*$extension" -File | ForEach-Object {
        $parentDirName = Split-Path (Split-Path $_.Directory -Parent) -Leaf
        $targetDir = Split-Path (Split-Path (Split-Path $_.Directory -Parent) -Parent) -Parent
        $newPath = Join-Path $targetDir "$parentDirName.$extension"

        if (-not (Test-Path $newPath)) {
            Move-Item $_.FullName $newPath
        }
    }
}

Process-Files ".rs"

Write-Output "Setup completed successfully!"
