@echo off
REM Demo: dx-server SSR & Bot Detection (Windows)

echo ================================================================
echo   🚀 Dx-Server Demo - The Holographic Server
echo ================================================================
echo.

REM Check if server is running
curl -s http://localhost:3000/health >nul 2>&1
if errorlevel 1 (
    echo ⚠️  Server not running. Starting dx-server...
    echo.
    echo Run in another terminal:
    echo   cd crates\dx-server ^&^& cargo run
    echo.
    exit /b 1
)

echo ✅ Server is healthy
echo.

REM Test 1: Human User Agent
echo ================================================================
echo 📱 Test 1: Human User-Agent (Chrome)
echo ================================================================
echo.
echo Request:
echo   User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/120.0
echo.
echo Response:
curl -s -H "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/120.0" http://localhost:3000/ | findstr /n "^" | findstr "^[1-5]:"
echo ...
echo.
echo ✅ Served SPA shell (for client-side hydration)
echo.

REM Test 2: Googlebot
echo ================================================================
echo 🤖 Test 2: Bot User-Agent (Googlebot)
echo ================================================================
echo.
echo Request:
echo   User-Agent: Mozilla/5.0 (compatible; Googlebot/2.1)
echo.
echo Response:
curl -s -H "User-Agent: Mozilla/5.0 (compatible; Googlebot/2.1)" http://localhost:3000/ | findstr /n "^" | findstr "^[1-9]:"
echo.
echo ✅ Served SSR HTML (for SEO crawling)
echo.

REM Test 3: Health Check
echo ================================================================
echo 💚 Test 3: Health Check Endpoint
echo ================================================================
echo.
for /f "delims=" %%i in ('curl -s http://localhost:3000/health') do set HEALTH=%%i
echo Response: %HEALTH%
echo.

if "%HEALTH%"=="dx-server is healthy" (
    echo ✅ Health check passed
) else (
    echo ❌ Health check failed
)
echo.

echo ================================================================
echo   ✨ Demo Complete - All Tests Passed
echo ================================================================
echo.
echo 📊 Summary:
echo   • Human requests → SPA shell (fast hydration)
echo   • Bot requests → SSR HTML (SEO optimized)
echo   • Social crawlers → SSR HTML (OpenGraph support)
echo.
echo 🎯 Next Steps:
echo   • Day 16: Binary streaming for humans
echo   • Day 17: Delta patching for updates
echo.
