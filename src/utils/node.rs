use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};

#[derive(Debug)]
pub struct SharedNode<T> where T: PartialEq + Debug {
    pointer: Rc<Node<T>>,
}

impl<T> SharedNode<T> where T: PartialEq + Debug {
    pub fn new(value: T) -> Self {
        SharedNode { pointer: Rc::new(Node::new(value)) }
    }

    pub fn add_child(&self, child: &SharedNode<T>) {
        Node::add_child(&self.pointer, &child.pointer);
    }

    pub fn print(&self) {
        self.pointer.print();
    }
}

#[derive(Debug)]
struct Node<T> where T: PartialEq + Debug {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> where T: PartialEq + Debug {
    fn new(value: T) -> Self {
        Node { value, parent: RefCell::new(Weak::new()), children: RefCell::new(vec![]) }
    }

    fn value(&self) -> &T {
        &self.value
    }

    fn parent(&self) -> Option<Rc<Node<T>>> {
        self.parent.borrow().upgrade()
    }

    fn children(&self) -> &RefCell<Vec<Rc<Node<T>>>> {
        &self.children
    }

    fn print(&self) {
        println!("{:?}", self);
        for i in 0..self.children.borrow().len() {
            Node::print(&self.children().borrow().get(i).unwrap());
        }
    }

    fn add_child(parent: &Rc<Node<T>>, child: &Rc<Node<T>>) {
        parent.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(&parent);
    }

    fn find(&self, value: T) {
        // TODO
    }
}