use std::fs;

/// Copies the template from day01.rs to the 24 other days, and creates 25 empty input files.
fn main() {
    // Create src/bin directory if it doesn't exist
    fs::create_dir_all("src/bin").unwrap();

    // I manually wrote the template for Day 1, so just copy it over
    let day_content = include_str!("day01.rs");

    // Start at 2, because 1 was already done
    for day in 2..=25 {
        let input_content = "";

        // Write day file
        let day_filename = format!("src/bin/day{:02}.rs", day);
        fs::write(&day_filename, &day_content).unwrap();

        // Write input file
        let input_filename = format!("input/day{:02}.txt", day);
        fs::write(&input_filename, &input_content).unwrap();
    }

    println!("Generated 25 Advent of Code day files and inputs.");
}
