use std::fmt::{Display, Formatter};
use std::ops::Add;
use crate::utils::helper::read_file;

struct File {
    name: String,
    size: i32,
}

struct Directory {
    name: String,
    size: i32,
    files: Vec<File>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Directory { name: String::from(name), size: 0, files: Vec::new() }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file)
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("");

        for file in &self.files {
            let a = [file.size.to_string(), file.name.to_string(), "\n".to_string()].join(" ");
            str = str.add(a.as_str());
        }

        write!(f, "{}", str)
    }
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn part1() {
    let file = read_file("./src/input/day7.txt");
}