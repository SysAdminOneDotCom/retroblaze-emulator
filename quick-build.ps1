# Quick Build Script (After Visual Studio Build Tools are installed)

Write-Host "🚀 Quick Build - RetroBlazeEmulator" -ForegroundColor Cyan
Write-Host ""

# Just build release version quickly
Write-Host "Building release version..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ Build successful!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Run with: cargo run --release -- --system nes --rom game.nes" -ForegroundColor Cyan
} else {
    Write-Host ""
    Write-Host "❌ Build failed! Run .\build.ps1 for detailed diagnostics" -ForegroundColor Red
    exit 1
}
