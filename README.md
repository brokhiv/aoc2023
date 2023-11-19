# aoc-2023

It is that time of the year again, and this time I'm doing Advent of Code in Rust!

This is my first project in the language, and I am eager to learn Rust with the AoC.

## Usage
To easily run a day or all days, I have created some batch scripts:
 - `run_all_days` builds and runs all days in release mode.
 - `run_current_day` builds and runs the current day of the month in debug mode. Mainly used during the AoC itself, and will not work outside the 1-25 day range.
 - `run_day` builds and runs the given day in debug mode. The argument should be in two digits.

## Acknowledgements
I am using the crate `advtools` by birkenfeld to reduce boilerplate, 
and `aocdl` by GreenLightning to automatically download my puzzle inputs every day.