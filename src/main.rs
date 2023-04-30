use crate::utils::node::Node;

mod days;
mod utils;

fn main() {
    days::day1::part1();
    days::day1::part2();
    days::day2::part1();
    days::day2::part2();
    days::day3::part1();
    days::day3::part2();
    days::day4::part1();
    days::day4::part2();
    days::day5::part1();
    days::day5::part2();
    days::day6::part1();
    days::day6::part2();
    days::day7::part1();

    let mut root = Node::new(10);
    let mut node2 = Node::new(20);
    node2.add_child(Node::new(25));
    root.add_child(Node::new(30));
    root.add_child(node2);

}
