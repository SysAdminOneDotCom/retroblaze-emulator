# RetroBlazeEmulator - Post-Restart Build Script
# Run this after restarting your computer

Write-Host "🎮 RetroBlazeEmulator - Build Script" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "❌ Error: Not in the gameemulator directory!" -ForegroundColor Red
    Write-Host "Please run this script from: c:\git\gameemulator" -ForegroundColor Yellow
    exit 1
}

# Verify Visual Studio Build Tools are installed
Write-Host "🔍 Checking for Visual Studio Build Tools..." -ForegroundColor Yellow
$linkExe = Get-Command link.exe -ErrorAction SilentlyContinue

if ($null -eq $linkExe) {
    Write-Host "⚠️  Warning: link.exe not found in PATH" -ForegroundColor Red
    Write-Host "   Searching in common Visual Studio locations..." -ForegroundColor Yellow
    
    $vsPath = "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC"
    if (Test-Path $vsPath) {
        $msvcVersion = Get-ChildItem $vsPath | Sort-Object Name -Descending | Select-Object -First 1
        $linkPath = "$($msvcVersion.FullName)\bin\Hostx64\x64"
        Write-Host "   Found MSVC at: $linkPath" -ForegroundColor Green
        Write-Host ""
        Write-Host "⚠️  You may need to restart your terminal/computer for PATH to update" -ForegroundColor Yellow
        Write-Host "   Or run this to add it temporarily:" -ForegroundColor Yellow
        Write-Host "   `$env:PATH += `";$linkPath`"" -ForegroundColor Cyan
        Write-Host ""
        Read-Host "Press Enter to continue anyway (build might fail)"
    } else {
        Write-Host "❌ Visual Studio Build Tools not found!" -ForegroundColor Red
        Write-Host "   Please install them using:" -ForegroundColor Yellow
        Write-Host "   winget install Microsoft.VisualStudio.2022.BuildTools" -ForegroundColor Cyan
        exit 1
    }
} else {
    Write-Host "✅ Visual Studio Build Tools found: $($linkExe.Source)" -ForegroundColor Green
}

Write-Host ""
Write-Host "📦 Checking Rust installation..." -ForegroundColor Yellow
$cargo = Get-Command cargo -ErrorAction SilentlyContinue

if ($null -eq $cargo) {
    Write-Host "❌ Cargo not found! Please install Rust from: https://rustup.rs" -ForegroundColor Red
    exit 1
} else {
    $rustVersion = cargo --version
    Write-Host "✅ Rust installed: $rustVersion" -ForegroundColor Green
}

Write-Host ""
Write-Host "🔨 Starting build process..." -ForegroundColor Yellow
Write-Host "   This will take 5-10 minutes on first build" -ForegroundColor Gray
Write-Host "   (Subsequent builds will be much faster)" -ForegroundColor Gray
Write-Host ""

# Run cargo check first (faster than full build)
Write-Host "1️⃣  Running cargo check..." -ForegroundColor Cyan
$checkResult = cargo check 2>&1
$checkExitCode = $LASTEXITCODE

if ($checkExitCode -eq 0) {
    Write-Host "✅ Cargo check passed!" -ForegroundColor Green
} else {
    Write-Host "❌ Cargo check failed!" -ForegroundColor Red
    Write-Host $checkResult
    Write-Host ""
    Write-Host "Common fixes:" -ForegroundColor Yellow
    Write-Host "  1. Restart your computer if you just installed VS Build Tools" -ForegroundColor Gray
    Write-Host "  2. Close and reopen this terminal" -ForegroundColor Gray
    Write-Host "  3. Run: rustup update" -ForegroundColor Gray
    exit 1
}

Write-Host ""
Write-Host "2️⃣  Building debug version..." -ForegroundColor Cyan
cargo build
$debugExitCode = $LASTEXITCODE

if ($debugExitCode -eq 0) {
    Write-Host "✅ Debug build successful!" -ForegroundColor Green
} else {
    Write-Host "❌ Debug build failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "3️⃣  Building release version (optimized)..." -ForegroundColor Cyan
Write-Host "   This will take longer but produces a fast executable" -ForegroundColor Gray
cargo build --release
$releaseExitCode = $LASTEXITCODE

if ($releaseExitCode -eq 0) {
    Write-Host "✅ Release build successful!" -ForegroundColor Green
} else {
    Write-Host "❌ Release build failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "🎉 BUILD COMPLETE!" -ForegroundColor Green
Write-Host "==================" -ForegroundColor Green
Write-Host ""
Write-Host "Executable locations:" -ForegroundColor Cyan
Write-Host "  Debug:   target\debug\retroblaze-emulator.exe" -ForegroundColor Gray
Write-Host "  Release: target\release\retroblaze-emulator.exe" -ForegroundColor Gray
Write-Host ""
Write-Host "📖 Next Steps:" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. Get a NES ROM file (you must own the original game)" -ForegroundColor White
Write-Host ""
Write-Host "2. Run the emulator:" -ForegroundColor White
Write-Host "   cargo run --release -- --system nes --rom `"path\to\game.nes`"" -ForegroundColor Cyan
Write-Host ""
Write-Host "3. Connect your PlayStation DualShock 4 controller (optional)" -ForegroundColor White
Write-Host "   - USB: Just plug it in" -ForegroundColor Gray
Write-Host "   - Bluetooth: Hold PS+Share buttons for 3 seconds" -ForegroundColor Gray
Write-Host ""
Write-Host "4. Controls:" -ForegroundColor White
Write-Host "   - ESC: Quit" -ForegroundColor Gray
Write-Host "   - F5: Save state" -ForegroundColor Gray
Write-Host "   - F9: Load state" -ForegroundColor Gray
Write-Host "   - F11: Fullscreen" -ForegroundColor Gray
Write-Host "   - P: Pause" -ForegroundColor Gray
Write-Host ""
Write-Host "📚 Documentation:" -ForegroundColor Yellow
Write-Host "   - README.md: Project overview" -ForegroundColor Gray
Write-Host "   - QUICKSTART.md: Detailed usage guide" -ForegroundColor Gray
Write-Host "   - PROJECT_SUMMARY.md: Complete project info" -ForegroundColor Gray
Write-Host ""
Write-Host "Happy retro gaming! 🎮" -ForegroundColor Magenta
