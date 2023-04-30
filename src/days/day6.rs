use crate::utils::helper::*;

pub fn part1() {
    do_magic(4);
}

pub fn part2(){
    do_magic(14);
}

fn do_magic(distinct: usize) {
    let file = read_file("./src/input/day6.txt");
    let buffer = file.get(0).unwrap();
    let len = buffer.chars().count();

    for i in 0..len {
        let mut upper = i + distinct;
        upper = if upper > len {len} else {upper};
        let subset = &buffer[i..upper];

        if is_unique(&subset.to_string()) {
            println!("{}", upper);
            break;
        }
    }
}

fn is_unique(str: &String) -> bool{
    let mut is_unique = true;
    for char in str.chars() {
        if str.matches(char).count() > 1 {
            is_unique = false;
        };
    };
    return is_unique;
}