# Build script for Secure Messaging Protocol
# This script helps with development and testing

param(
    [string]$Action = "help",
    [string]$ClientName = "test"
)

function Show-Help {
    Write-Host "🔐 Secure Messaging Protocol Build Script" -ForegroundColor Green
    Write-Host "==========================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Usage: .\build.ps1 -Action <action> [-ClientName <name>]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Actions:" -ForegroundColor Cyan
    Write-Host "  build     - Build the project" -ForegroundColor White
    Write-Host "  test      - Run tests" -ForegroundColor White
    Write-Host "  server    - Start the server" -ForegroundColor White
    Write-Host "  client    - Start a client (use -ClientName to specify name)" -ForegroundColor White
    Write-Host "  demo      - Run the Python demo" -ForegroundColor White
    Write-Host "  clean     - Clean build artifacts" -ForegroundColor White
    Write-Host "  help      - Show this help" -ForegroundColor White
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Yellow
    Write-Host "  .\build.ps1 -Action build" -ForegroundColor White
    Write-Host "  .\build.ps1 -Action server" -ForegroundColor White
    Write-Host "  .\build.ps1 -Action client -ClientName alice" -ForegroundColor White
    Write-Host "  .\build.ps1 -Action demo" -ForegroundColor White
}

function Build-Project {
    Write-Host "🔨 Building project..." -ForegroundColor Green
    try {
        cargo build --release
        Write-Host "✅ Build successful!" -ForegroundColor Green
    }
    catch {
        Write-Host "❌ Build failed: $_" -ForegroundColor Red
    }
}

function Test-Project {
    Write-Host "🧪 Running tests..." -ForegroundColor Green
    try {
        cargo test
        Write-Host "✅ Tests completed!" -ForegroundColor Green
    }
    catch {
        Write-Host "❌ Tests failed: $_" -ForegroundColor Red
    }
}

function Start-Server {
    Write-Host "🚀 Starting server..." -ForegroundColor Green
    try {
        cargo run --bin server
    }
    catch {
        Write-Host "❌ Failed to start server: $_" -ForegroundColor Red
    }
}

function Start-Client {
    Write-Host "📱 Starting client '$ClientName'..." -ForegroundColor Green
    try {
        cargo run --bin client $ClientName
    }
    catch {
        Write-Host "❌ Failed to start client: $_" -ForegroundColor Red
    }
}

function Run-Demo {
    Write-Host "🎭 Running Python demo..." -ForegroundColor Green
    try {
        python test_demo.py
    }
    catch {
        Write-Host "❌ Failed to run demo: $_" -ForegroundColor Red
    }
}

function Clean-Project {
    Write-Host "🧹 Cleaning project..." -ForegroundColor Green
    try {
        cargo clean
        if (Test-Path "data") {
            Remove-Item -Recurse -Force "data"
        }
        Write-Host "✅ Clean completed!" -ForegroundColor Green
    }
    catch {
        Write-Host "❌ Clean failed: $_" -ForegroundColor Red
    }
}

# Main execution
switch ($Action.ToLower()) {
    "build" { Build-Project }
    "test" { Test-Project }
    "server" { Start-Server }
    "client" { Start-Client }
    "demo" { Run-Demo }
    "clean" { Clean-Project }
    "help" { Show-Help }
    default { 
        Write-Host "❌ Unknown action: $Action" -ForegroundColor Red
        Show-Help
    }
} 