use std::collections::VecDeque;
use std::fmt::Display;
use crate::utils::generic::node::Node;

pub(crate) struct Tree<'t, T> where T: Display + PartialEq {
    root: &'t Node<T>,
}

#[allow(unused)]
impl<'t, T> Tree<'t, T> where T: Display + PartialEq {
    pub fn from(root: &'t Node<T>) -> Self {
        Self { root }
    }

    fn traverse(&self, mut process: impl FnMut(&'t Node<T>) -> Option<& Node<T>>) {
        let mut stack: VecDeque<&Node<T>> = VecDeque::new();
        stack.push_back(&self.root);

        while !stack.is_empty() {
            let node = stack.pop_front().unwrap();
            if let Some(_) = process(&node) {
                break;
            }

            for i in 0..node.children().len() {
                stack.push_back(node.children().get(i).unwrap());
            }
        }
    }

    pub fn find(&self, value: T) -> Option<&'t Node<T>> {
        let mut ret = None;
        self.traverse(|n| {
            if n.value() == &value {
                ret = Some(n);
            }
            None
        });
        ret
    }

    /*fn traverse_mut(&self, mut process: impl FnMut(&'t mut Node<T>) -> Option<&'t mut Node<T>>) {
        let mut stack: VecDeque<&Node<T>> = VecDeque::new();
        stack.push_back(&self.root);

        while !stack.is_empty() {
            let mut a = stack.pop_front();
            let node = a.as_mut().unwrap();
            if let Some(_) = process(node) {
                break;
            }

            for i in 0..node.children().len() {
                stack.push_back(node.children().get(i).unwrap());
            }
        }
    }*/

    pub fn find_mut(&self, value: T) -> Option<&'t mut Node<T>> {
        None
    }

    /*pub fn add_node(&self, node: Node<T>, parent: T) {
        if let Some(found) = self.find(parent).as_mut() {
            found.add_child(node);
        }
    }*/

}