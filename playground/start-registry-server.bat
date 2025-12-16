@echo off
REM Start DX Registry Server

cd /d "F:\Code\dx\crates\dx-package-manager"

echo Starting DX Registry Server on localhost:3000...
echo.

start "DX Registry Server" target\release\dx-pkg-registry-server.exe .dx-registry 127.0.0.1:3000

timeout /t 2 /nobreak >nul

echo.
echo Server should be running in a new window
echo Press any key to stop the server...
pause

taskkill /FI "WINDOWTITLE eq DX Registry Server*" /F
