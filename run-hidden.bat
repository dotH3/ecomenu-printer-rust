@echo off
cd /d "%~dp0"
powershell -WindowStyle Hidden -Command "Start-Process 'ecomenu-printer.exe' -WindowStyle Hidden"
exit
