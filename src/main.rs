use crate::utils::node::{Node};
mod days;
mod utils;

fn main() {
    let root = Node::new(10);

    let ayyy = Node::new(20);

    root.add_child(&ayyy);

    println!("{:?}", ayyy.parent());
}
