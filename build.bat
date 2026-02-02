@echo off
REM Quick build script for Windows
REM Sets up environment and builds the emulator

echo.
echo ========================================
echo   RetroBlazeEmulator - Windows Build
echo ========================================
echo.

REM Add CMake to PATH for this session
set "PATH=%PATH%;C:\Program Files\CMake\bin"

echo Checking CMake...
cmake --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: CMake not found!
    echo Please install CMake 3.30.5
    pause
    exit /b 1
)

echo.
echo Building release version...
echo This will take 5-10 minutes on first build.
echo.

cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ========================================
    echo   BUILD SUCCESSFUL!
    echo ========================================
    echo.
    echo Executable: target\release\retroblaze-emulator.exe
    echo.
    echo To run:
    echo   cargo run --release -- --system nes --rom game.nes
    echo.
) else (
    echo.
    echo ========================================
    echo   BUILD FAILED!
    echo ========================================
    echo.
    echo Check errors above
)

pause
