param(
    [string]$Name = "my_library"
)

cargo new --lib $Name
Set-Location $Name

Write-Host "Rust library created: $Name"