use std::cmp::{max, min};
use std::str::FromStr;
use crate::util::read_file;

fn get_bounds_of_line(line: &String) -> (i32, i32, i32, i32) {
    let elves: Vec<String> = line.split(",").map(|s| s.to_string()).collect();

    // split into "elves"
    let elf1 = elves.get(0).unwrap();
    let elf2 = elves.get(1).unwrap();

    let l: Vec<i32> = elf1.split("-").map(|s| FromStr::from_str(s).unwrap()).collect();
    let r: Vec<i32> = elf2.split("-").map(|s| FromStr::from_str(s).unwrap()).collect();
    (l.get(0).unwrap().clone(), l.get(1).unwrap().clone(), r.get(0).unwrap().clone(), r.get(1).unwrap().clone())
}

pub fn part1() {
    let mut count = 0;

    for line in read_file("./src/input/day4.txt") {
        let (lower_bound_l, upper_bound_l, lower_bound_r, upper_bound_r) = get_bounds_of_line(&line);

        if lower_bound_l <= lower_bound_r && upper_bound_l >= upper_bound_r ||
            lower_bound_r <= lower_bound_l && upper_bound_r >= upper_bound_l {
            count += 1;
        }
    }

    println!("{}", count);
}

pub fn part2() {
    let mut count = 0;

    for line in read_file("./src/input/day4.txt") {
        let (lower_bound_l, upper_bound_l, lower_bound_r, upper_bound_r) = get_bounds_of_line(&line);

        if max(lower_bound_l, lower_bound_r) <= min(upper_bound_l, upper_bound_r){
            count += 1;
        }
    }

    println!("{}", count);
}