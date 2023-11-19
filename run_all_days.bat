@echo off
setlocal enabledelayedexpansion

REM Run all Rust files in release mode
for %%f in (src\bin\day*.rs) do (
    set "day=%%~nf"
    echo Running day !day! in release mode
    cargo run --release --bin !day!
)

endlocal
