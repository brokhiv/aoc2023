use std::str::FromStr;
use advtools::input;

fn main() {
    let example = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let games = input::lines()
        .map(|line| line.split_once(": ").unwrap().1.split("; ").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (red, green, blue) = (12, 13, 14);
    let mut game_sum = 0;
    let mut i = 1;

    for game in &games {
        let mut impossible: bool = false;
        for round in game {
            for color in round.split(", ") {
                let (num, col) = color.split_once(" ").unwrap();
                let lim = match col {
                    "red" => &red,
                    "green" => &green,
                    "blue" => &blue,
                    _ => unreachable!()
                };
                if &i32::from_str(num).unwrap() > lim {
                    impossible = true;
                    break;
                }
            }
            if impossible { break; }
        }
        if !impossible { game_sum += &i; }
        i += 1;
    }

    advtools::print("a", &game_sum);

    let mut power_sum = 0;
    for game in &games {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for round in game {
            for color in round.split(", ") {
                let (num, col) = color.split_once(" ").unwrap();
                let val = i32::from_str(num).unwrap();
                match col {
                    "red" => red = red.max(val),
                    "green" => green = green.max(val),
                    "blue" => blue = blue.max(val),
                    _ => unreachable!()
                };
            }
        }
        power_sum += red * green * blue;
    }
    advtools::print("b", power_sum);
}