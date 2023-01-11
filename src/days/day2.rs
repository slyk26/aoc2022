// ROCK => 1
// PAPER => 2
// SCISSOR => 3
// WIN => 6
// DRAW => 3
// LOSE => 0

use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> Vec<String> {
    let file = File::open("./src/input/day2.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn lookup(key: &str) -> i32 {
    match key {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,

        &_ => -69420 // lol
    }
}

pub fn part1() {
    let mut score = 0;

    for line in read_file().iter() {
        let game = line.split(" ").collect::<Vec<&str>>();

        let elf = lookup(game.get(0).unwrap());
        let me = lookup(game.get(1).unwrap());

        let mut result;

        if (me == 1 && elf == 3) || (me == 2 && elf == 1) || (me == 3 && elf == 2) {
            // win
            result = 6;
        } else if me == elf {
            // draw
            result = 3;
        } else {
            // loss
            result = 0;
        }

        result += me;
        score += result;
    };

    println!("Score of R/P/S: {}", score);
}

pub fn part2() {
    let mut score = 0;

    for line in read_file().iter() {
        let game = line.split(" ").collect::<Vec<&str>>();

        let elf = lookup(game.get(0).unwrap());
        let outcome = game.get(1).unwrap();
        let mut result = 0;

        match *outcome {
            "Y" => {
                // draw
                result = 3 + elf;
            }

            "X" => {
                // lose
                result = match elf {
                    1 => 3, // rock => scissors
                    2 => 1, // paper => rock
                    _ => 2, // scissors => paper
                }
            }

            _ => {
                // win
                result = 6;

                result += match elf {
                    1 => 2, // rock => paper
                    2 => 3, // paper => scissors
                    _ => 1 // scissors => rock
                }

            }
        }
        score += result;
    }

    println!("Part 2 R/P/S{}", score);
}
