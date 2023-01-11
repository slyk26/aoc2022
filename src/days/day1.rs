use std::collections::{BTreeMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_elves() -> BTreeMap<i32, usize> {
    let file = File::open("./src/days/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut elves:BTreeMap<i32, usize> = BTreeMap::new();

    let mut elf: i32 = 0;
    for (index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        if l.is_empty() {
            elves.insert(elf, index);
            elf = 0;
        } else {
            let cal = l.parse::<i32>().unwrap();
            elf += cal;
        }
    };
    elves
}

pub fn part1() {
    let mut elves = get_elves();
    let elf = elves.last_entry().unwrap();
    println!("most Calories on elf #{}: {} Calories", elf.get(), elf.key());
}

pub fn part2() {
    let mut elves = get_elves();
    let top_three = vec![elves.pop_last().unwrap().0, elves.pop_last().unwrap().0, elves.pop_last().unwrap().0];
    let sum: i32 = top_three.iter().sum();
    println!("Top Three have {} Calories", sum);
}