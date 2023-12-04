extern crate core;

use std::iter;
use std::str::FromStr;
use advtools::input;
use advtools::itertools::Itertools;

fn main() {
    let example = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let parsed_input: Vec<(Vec<i32>, Vec<i32>)> = input::lines().map(|line| line
        .split_once(": ")
        .and_then(|(_, x)| x.split_once(" | "))
        .map(|(ws, ns)| (
            ws.split_whitespace().map(|w| i32::from_str(w).unwrap()).collect(),
            ns.split_whitespace().map(|n| i32::from_str(n).unwrap()).collect())
        ).unwrap()
    ).collect();

    let mut sum_a = 0;
    let mut copies: Vec<i32> = iter::repeat(1).take(parsed_input.len()).collect();
    for (i, (ws, ns)) in parsed_input.iter().enumerate() {
        let mut wins = 0;
        for n in ns {
            if ws.contains(&n) {
                wins += 1;
            }
        }
        let range = i.clone() + 1..i.clone() + wins.clone()+1;
        for j in range {
            copies[j] += copies[i];
        };
        sum_a += if wins == 0 { 0 } else { 1 << (wins-1) };
    }
    let sum_b: i32 = copies.iter().sum();

    // debug_assert_eq!(13, sum_a);
    advtools::print("a", sum_a);

    // debug_assert_eq!(30, sum_b);
    advtools::print("b", sum_b);
}