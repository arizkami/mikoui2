@echo off
REM Build script wrapper for Rabital
REM Usage: build.bat [debug|release]

set CONFIG=%1
if "%CONFIG%"=="" set CONFIG=release

powershell -ExecutionPolicy Bypass -File build-app.ps1 -Config %CONFIG%
