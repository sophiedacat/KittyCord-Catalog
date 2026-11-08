@echo off
:: ============================================================================
! KittyCord-Catalog - Install Dependencies
! Installs Rust toolchain with Wasm target and Python required for building
! KittyCord plugins.
:: ============================================================================

echo [1/3] Checking for Rust...

where rustup >nul 2>&1
if %errorlevel% neq 0 (
    echo Rust not found. Installing rustup...
    powershell -NoProfile -ExecutionPolicy Bypass -Command "& {iex((New-Object System.Net.WebClient).DownloadString('https://sh.rustup.rs'))}" -y --default-toolchain stable --profile minimal
) else (
    echo Rust is already installed.
)

echo.
echo [2/3] Adding WebAssembly target...

rustup target add wasm32-unknown-unknown

echo.
echo [3/3] Checking for Python...

where python >nul 2>&1
if %errorlevel% neq 0 (
    echo Python not found. Please install Python 3 from https://python.org
    echo and ensure 'python' is on your PATH, then re-run this script.
    echo.
    pause
    exit /b 1
) else (
    echo Python is already installed.
)

echo.
echo ============================================================================
echo All dependencies installed!
echo You can now run git-push.bat to build and push plugins to the catalog.
echo ============================================================================

pause
