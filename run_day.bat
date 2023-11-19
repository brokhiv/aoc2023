@echo off
setlocal

REM Check if a day number is provided as a parameter
if "%1"=="" (
    echo Please provide a day number as a parameter.
    exit /b 1
)

REM Run the Rust file for the specified day in debug mode
set "target_file=src\bin\day%1.rs"

if exist "%target_file%" (
    echo Running %target_file%
    cargo run --bin day%1
) else (
    echo No Rust file found for day %1
)

endlocal
