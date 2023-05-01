use crate::utils::node::{SharedNode};

mod days;
mod utils;

fn main() {
    let root = SharedNode::new(10);

    let ayyy = SharedNode::new(20);
    
    root.add_child(&ayyy);

    root.print();
}
