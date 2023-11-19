@echo off
setlocal

REM Get the current day of the month
for /f "delims=" %%d in ('powershell Get-Date -Format dd') do set "current_day=%%d"

REM Run the Rust file for the current day in debug mode
set "target_file=src\bin\day%current_day%.rs"

if exist "%target_file%" (
    echo Running %target_file%
    cargo run --bin day%current_day%
) else (
    echo No Rust file found for day %current_day%
)

endlocal
