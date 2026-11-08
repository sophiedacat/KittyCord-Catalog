@echo off
:: ============================================================================
:: KittyCord-Catalog - Build, Package & Push
:: Builds all 6 plugins to Wasm, packages them, updates catalog.json,
! and pushes everything to GitHub.
:: ============================================================================

setlocal enabledelayedexpansion
set ROOT=%~dp0
cd /d "%ROOT%"

:: Add Rust to PATH if not already there
where rustc >nul 2>&1
if %errorlevel% neq 0 (
    set PATH=%PATH%;%USERPROFILE%\.cargo\bin
)

echo ============================================================================
echo KittyCord-Catalog - Build & Push
echo ============================================================================
echo.

:: Check for Rust
where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Rust not found. Run install-deps.bat first.
    pause
    exit /b 1
)

:: Check for Python
where python >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Python not found. Run install-deps.bat first.
    pause
    exit /b 1
)

:: Check for Wasm target
rustup target list --installed 2>nul | findstr wasm32-unknown-unknown >nul
if %errorlevel% neq 0 (
    echo Adding wasm32-unknown-unknown target...
    rustup target add wasm32-unknown-unknown
    if %errorlevel% neq 0 (
        echo ERROR: Failed to add wasm32-unknown-unknown target.
        pause
        exit /b 1
    )
)

echo.
echo [1/6] Building message-logger...
cd /d "%ROOT%extensions\plugins\message-logger"
cargo build --release --target wasm32-unknown-unknown
if %errorlevel% neq 0 (
    echo ERROR: Failed to build message-logger
    pause
    exit /b 1
)

echo.
echo [2/6] Building status-logger...
cd /d "%ROOT%extensions\plugins\status-logger"
cargo build --release --target wasm32-unknown-unknown
if %errorlevel% neq 0 (
    echo ERROR: Failed to build status-logger
    pause
    exit /b 1
)

echo.
echo [3/6] Building relation-logger...
cd /d "%ROOT%extensions\plugins\relation-logger"
cargo build --release --target wasm32-unknown-unknown
if %errorlevel% neq 0 (
    echo ERROR: Failed to build relation-logger
    pause
    exit /b 1
)

echo.
echo [4/6] Building anti-ghost...
cd /d "%ROOT%extensions\plugins\anti-ghost"
cargo build --release --target wasm32-unknown-unknown
if %errorlevel% neq 0 (
    echo ERROR: Failed to build anti-ghost
    pause
    exit /b 1
)

echo.
echo [5/6] Building auto-reply...
cd /d "%ROOT%extensions\plugins\auto-reply"
cargo build --release --target wasm32-unknown-unknown
if %errorlevel% neq 0 (
    echo ERROR: Failed to build auto-reply
    pause
    exit /b 1
)

echo.
echo [6/6] Building bulk-delete...
cd /d "%ROOT%extensions\plugins\bulk-delete"
cargo build --release --target wasm32-unknown-unknown
if %errorlevel% neq 0 (
    echo ERROR: Failed to build bulk-delete
    pause
    exit /b 1
)

echo.
echo ============================================================================
echo All plugins built successfully!
echo ============================================================================

echo.
echo Packaging plugins...

cd /d "%ROOT%extensions"

python pack.py plugins/message-logger/manifest.json target/wasm32-unknown-unknown/release/message_logger.wasm plugins/packages/message-logger.serein-extension
python pack.py plugins/status-logger/manifest.json target/wasm32-unknown-unknown/release/status_logger.wasm plugins/packages/status-logger.serein-extension
python pack.py plugins/relation-logger/manifest.json target/wasm32-unknown-unknown/release/relation_logger.wasm plugins/packages/relation-logger.serein-extension
python pack.py plugins/anti-ghost/manifest.json target/wasm32-unknown-unknown/release/anti_ghost.wasm plugins/packages/anti-ghost.serein-extension
python pack.py plugins/auto-reply/manifest.json target/wasm32-unknown-unknown/release/auto_reply.wasm plugins/packages/auto-reply.serein-extension
python pack.py plugins/bulk-delete/manifest.json target/wasm32-unknown-unknown/release/bulk_delete.wasm plugins/packages/bulk-delete.serein-extension

echo.
echo Copying built packages to catalog...

copy "C:\Users\oxyiscute\Downloads\Serein-main\extensions\plugins\packages\message-logger.serein-extension" "C:\Users\oxyiscute\Downloads\KittyCord-Catalog\plugins\packages\message-logger.serein-extension"
copy "C:\Users\oxyiscute\Downloads\Serein-main\extensions\plugins\packages\status-logger.serein-extension" "C:\Users\oxyiscute\Downloads\KittyCord-Catalog\plugins\packages\status-logger.serein-extension"
copy "C:\Users\oxyiscute\Downloads\Serein-main\extensions\plugins\packages\relation-logger.serein-extension" "C:\Users\oxyiscute\Downloads\KittyCord-Catalog\plugins\packages\relation-logger.serein-extension"
copy "C:\Users\oxyiscute\Downloads\Serein-main\extensions\plugins\packages\anti-ghost.serein-extension" "C:\Users\oxyiscute\Downloads\KittyCord-Catalog\plugins\packages\anti-ghost.serein-extension"
copy "C:\Users\oxyiscute\Downloads\Serein-main\extensions\plugins\packages\auto-reply.serein-extension" "C:\Users\oxyiscute\Downloads\KittyCord-Catalog\plugins\packages\auto-reply.serein-extension"
copy "C:\Users\oxyiscute\Downloads\Serein-main\extensions\plugins\packages\bulk-delete.serein-extension" "C:\Users\oxyiscute\Downloads\KittyCord-Catalog\plugins\packages\bulk-delete.serein-extension"

echo.
echo Updating catalog.json with real hashes...

cd /d "%ROOT%"
python update-catalog.py

echo.
echo Committing and pushing to GitHub...

git add -A
git commit -m "chore: rebuild and publish all plugin packages with real SHA256 hashes"

git push origin master

if %errorlevel% neq 0 (
    echo ERROR: Failed to push to GitHub.
    pause
    exit /b 1
)

echo.
echo ============================================================================
echo Done! Catalog pushed to GitHub successfully.
echo ============================================================================

pause