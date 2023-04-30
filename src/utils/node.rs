use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ptr::NonNull;

pub(crate) struct Node<T> where T: Display + PartialEq {
    value: T,
    parent: Option<NonNull<Node<T>>>,
    children: Vec<Node<T>>,
}

#[allow(unused)]
impl<T> Node<T> where T: Display + PartialEq {
    pub fn new(value: T) -> Self {
        Self { value, children: Vec::new(), parent: None }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn parent(&self) -> &Option<NonNull<Node<T>>> {
        &self.parent
    }

    pub fn children(&self) -> &Vec<Node<T>> {
        &self.children
    }

    pub fn add_child(&mut self, mut child: Node<T>) {
        child.parent = Some(NonNull::from(&*self));
        self.children.push(child);
    }

    fn bfs(&self, f: impl Fn(&Node<T>)) {
        let mut q = VecDeque::new();
        q.push_back(self);

        while let Some(t) = q.pop_front() {
            (f)(&t);
            for child in &t.children {
                q.push_back(child);
            }
        }
    }

    pub fn print(&self) {
        self.bfs(|node| println!("{}", node));
    }

    pub fn find(&self, value: T) -> Option<&Node<T>> {
        let mut ret = None;
        self.bfs(| node| {
            if node.value == value {
                ret = Some(node);
            }
        });
        ret
    }

}

impl<T> Display for Node<T> where T: Display + PartialEq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            if let Some(parent) = self.parent {
                write!(f, "Parent({}) Node({})", parent.as_ref().value, self.value)
            } else {
                write!(f, "Node({})", self.value)
            }
        }
    }
}

impl<T> PartialEq for Node<T> where T: Display + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}