# Download popular homebrew ROMs for NES, SNES, and Genesis
# All ROMs are legal homebrew/public domain games

$romsDir = "roms"
if (-not (Test-Path $romsDir)) {
    New-Item -ItemType Directory -Path $romsDir
}

Write-Host "Downloading NES Homebrew ROMs..." -ForegroundColor Cyan

# NES ROMs
$nesRoms = @{
    "240pee.nes" = "https://github.com/pinobatch/240p-test-mini/releases/download/0.21/240pee.nes"
    "hello.nes" = "https://raw.githubusercontent.com/leoncool/learning-to-code/main/NES/hello/hello.nes"
}

foreach ($rom in $nesRoms.GetEnumerator()) {
    $filePath = Join-Path $romsDir $rom.Key
    if (-not (Test-Path $filePath)) {
        Write-Host "  Downloading $($rom.Key)..." -ForegroundColor Yellow
        try {
            Invoke-WebRequest -Uri $rom.Value -OutFile $filePath -UseBasicParsing
            Write-Host "    ✓ Downloaded $($rom.Key)" -ForegroundColor Green
        } catch {
            Write-Host "    ✗ Failed to download $($rom.Key): $_" -ForegroundColor Red
        }
    } else {
        Write-Host "  ✓ $($rom.Key) already exists" -ForegroundColor Green
    }
}

Write-Host "`nDownloading SNES Homebrew ROMs..." -ForegroundColor Cyan

# SNES ROMs (homebrew)
$snesRoms = @{
    "240p-test-suite.sfc" = "https://github.com/pinobatch/240p-test-mini/releases/download/0.21/240pee_mb.sfc"
}

foreach ($rom in $snesRoms.GetEnumerator()) {
    $filePath = Join-Path $romsDir $rom.Key
    if (-not (Test-Path $filePath)) {
        Write-Host "  Downloading $($rom.Key)..." -ForegroundColor Yellow
        try {
            Invoke-WebRequest -Uri $rom.Value -OutFile $filePath -UseBasicParsing
            Write-Host "    ✓ Downloaded $($rom.Key)" -ForegroundColor Green
        } catch {
            Write-Host "    ✗ Failed to download $($rom.Key): $_" -ForegroundColor Red
        }
    } else {
        Write-Host "  ✓ $($rom.Key) already exists" -ForegroundColor Green
    }
}

Write-Host "`nDownloading Genesis Homebrew ROMs..." -ForegroundColor Cyan

# Genesis ROMs (homebrew)
$genesisRoms = @{
    "240p-test-suite.gen" = "https://github.com/pinobatch/240p-test-mini/releases/download/0.21/240pee_sg.gen"
}

foreach ($rom in $genesisRoms.GetEnumerator()) {
    $filePath = Join-Path $romsDir $rom.Key
    if (-not (Test-Path $filePath)) {
        Write-Host "  Downloading $($rom.Key)..." -ForegroundColor Yellow
        try {
            Invoke-WebRequest -Uri $rom.Value -OutFile $filePath -UseBasicParsing
            Write-Host "    ✓ Downloaded $($rom.Key)" -ForegroundColor Green
        } catch {
            Write-Host "    ✗ Failed to download $($rom.Key): $_" -ForegroundColor Red
        }
    } else {
        Write-Host "  ✓ $($rom.Key) already exists" -ForegroundColor Green
    }
}

Write-Host "`n✓ ROM download complete!" -ForegroundColor Green
Write-Host "Launch the emulator to see your game library." -ForegroundColor Cyan
