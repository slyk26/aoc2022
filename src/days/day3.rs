use std::collections::HashSet;
use crate::utils::helper::*;

pub fn part1() {
    let mut sum = 0;

    for line in read_file("./src/input/day3.txt").iter() {
        let half = line.len() / 2;
        let first = &line[0..half];
        let second = &line[half..];
        let mut both: HashSet<char> = HashSet::new();

        for char in first.chars() {
            if second.contains(char) && !both.contains(&char) {
                if char.is_ascii_lowercase() {
                    sum += (char as u32) - 96;
                } else {
                    sum += (char as u32) - 38;
                }
                both.insert(char);
            }
        }
    }
    println!("{}", sum);
}

pub fn part2() {
    let mut sum = 0;
    let lines = read_file("./src/input/day3.txt");
    let groups = lines.chunks_exact(3);
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    for group in groups {
        for char in alphabet.chars() {
            if group[0].contains(char) && group[1].contains(char) && group[2].contains(char) {
                if char.is_ascii_lowercase() {
                    sum += (char as u32) - 96;
                } else {
                    sum += (char as u32) - 38;
                }
            }
        }
    }

    println!("{}", sum);

}