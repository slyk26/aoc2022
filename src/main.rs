use crate::utils::node::{Node};
mod days;
mod utils;

fn main() {
    let root = Node::new(10);
    let ayyy = Node::new(20);

    root.add_child(&ayyy);

    root.find(&20).unwrap().add_child(&Node::new(22));

    println!("{:?}", root);
}
