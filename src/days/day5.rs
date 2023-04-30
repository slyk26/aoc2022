use std::collections::VecDeque;
use crate::utils::helper::*;

pub fn part1() {
    solve_with(crate_mover9000);
}

pub fn part2() {
    solve_with(crate_mover9001);
}

fn solve_with(crate_mover: fn(stacks: &mut Vec<VecDeque<char>>, instructions: &[String]) -> ()) {
    let file = read_file("./src/input/day5.txt");
    let end_of_stacks = file.iter().position(|line| line.starts_with(' ')).unwrap();
    let mut stacks = get_stacks(&file[0..end_of_stacks]);

    crate_mover(&mut stacks, &file[end_of_stacks + 2..]);
    print_solution(&mut stacks);
}

fn get_stacks(crate_part: &[String]) -> Vec<VecDeque<char>> {
    let stacksize = get_stacksize(crate_part.get(0).unwrap());
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(stacksize);

    for _ in 0..stacksize {
        stacks.push(VecDeque::new());
    }

    for i in 0..stacksize - 1 {
        let line_as_chars = crate_part.get(i).unwrap().chars().collect::<Vec<char>>();
        let crate_chunks = line_as_chars.chunks(4);
        for (i, chunk) in crate_chunks.into_iter().enumerate() {
            if let Some(stack) = stacks.get_mut(i) {
                if chunk[1] != ' ' {
                    stack.push_front(chunk[1]);
                }
            }
        };
    }
    stacks
}

fn get_stacksize(a_line: &String) -> usize {
    a_line.chars().collect::<Vec<char>>().chunks(4).count()
}

fn crate_mover9000(stacks: &mut Vec<VecDeque<char>>, instructions: &[String]) {
    instructions.iter().for_each(|instruction| {
        let (amount, from, to) = parse_instruction(&instruction);

        for _ in 0..amount {
            let char = stacks.get_mut(from).unwrap().pop_back().unwrap();
            stacks.get_mut(to).unwrap().push_back(char);
        }
    })
}

fn crate_mover9001(stacks: &mut Vec<VecDeque<char>>, instructions: &[String]) {
    instructions.iter().for_each(|instruction| {
        let (amount, from, to) = parse_instruction(&instruction);
        let mut temp: VecDeque<char> = VecDeque::new();

        for _ in 0..amount {
            let char = stacks.get_mut(from).unwrap().pop_back().unwrap();
            temp.push_front(char);
        }
        stacks.get_mut(to).unwrap().append(&mut temp);
    })
}

fn parse_instruction(instruction: &String) -> (i32, usize, usize) {
    let words: Vec<&str> = instruction.split(' ').collect();
    (words[1].parse::<i32>().unwrap(), words[3].parse::<usize>().unwrap() - 1, words[5].parse::<usize>().unwrap() - 1)
}

fn print_solution(stacks: &mut Vec<VecDeque<char>>) {
    for stack in stacks {
        print!("{}", stack.pop_back().unwrap());
    }
    println!();
}