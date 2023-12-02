use advtools::input;
use advtools::prelude::Regex;

fn main() {
    let numbers = input::lines().map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>());
    let mut sum = 0;
    for number in numbers {
        sum += number[0].to_digit(10).unwrap() * 10;
        sum += number.last().unwrap().to_digit(10).unwrap();
    }

    advtools::print("a", sum);

    let example = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    sum = 0;
    for line in input::lines() {
        let mut first = 10;
        let mut last = 10;
        for i in 0..line.len() {
            let mut digit = 10;

            if line.chars().nth(i).unwrap().is_digit(10) {
                digit = line.chars().nth(i.clone()).unwrap().to_digit(10).unwrap();
            }
            else if &i + 4 <= line.len() && &line[i..i.clone()+4] == "zero" { digit = 0; }
            else if &i + 3 <= line.len() && &line[i..i.clone()+3] == "one" { digit = 1; }
            else if &i + 3 <= line.len() && &line[i..i.clone()+3] == "two" { digit = 2; }
            else if &i + 5 <= line.len() && &line[i..i.clone()+5] == "three" { digit = 3; }
            else if &i + 4 <= line.len() && &line[i..i.clone()+4] == "four" { digit = 4; }
            else if &i + 4 <= line.len() && &line[i..i.clone()+4] == "five" { digit = 5; }
            else if &i + 3 <= line.len() && &line[i..i.clone()+3] == "six" { digit = 6; }
            else if &i + 5 <= line.len() && &line[i..i.clone()+5] == "seven" { digit = 7; }
            else if &i + 5 <= line.len() && &line[i..i.clone()+5] == "eight" { digit = 8; }
            else if &i + 4 <= line.len() && &line[i..i.clone()+4] == "nine" { digit = 9; }
            else { continue; }

            if first == 10 { first = digit; }
            last = digit.clone();
        }
        sum += first * 10 + last;
    }

    advtools::print("b", sum);
}