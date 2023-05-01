use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};

#[derive(Debug)]
pub struct Node<T> where T: PartialEq + Debug {
    pointer: Rc<RawNode<T>>,
}

#[allow(unused)]
impl<T> Node<T> where T: PartialEq + Debug {
    pub fn new(value: T) -> Self {
        Node { pointer: Rc::new(RawNode::new(value)) }
    }

    fn from(pointer: &Rc<RawNode<T>>) -> Self {
        Node {pointer: Rc::clone(pointer)}
    }

    pub fn value(&self) -> &T {
        &self.pointer.value()
    }

    pub fn parent(&self) -> Option<Node<T>> {
        if let Some(parent) = &self.pointer.parent.borrow().upgrade(){
            return Some(Node::from(parent))
        }
        None
    }

    pub fn add_child(&self, child: &Node<T>) {
        RawNode::add_child(&self.pointer, &child.pointer);
    }

    pub fn print(&self) {
        self.pointer.print();
    }

    pub fn find(&self, value: &T) -> Option<Node<T>> {
        if let Some(found) = self.pointer.find(&value) {
            return Some(Node::from(&found));
        }
        None
    }
}

#[derive(Debug)]
struct RawNode<T> where T: PartialEq + Debug {
    value: T,
    parent: RefCell<Weak<RawNode<T>>>,
    children: RefCell<Vec<Rc<RawNode<T>>>>,
}

#[allow(unused)]
impl<T> RawNode<T> where T: PartialEq + Debug {
    fn new(value: T) -> Self {
        RawNode { value, parent: RefCell::new(Weak::new()), children: RefCell::new(vec![]) }
    }

    fn value(&self) -> &T {
        &self.value
    }

    fn parent(&self) -> Option<Rc<RawNode<T>>> {
        self.parent.borrow().upgrade()
    }

    fn children(&self) -> &RefCell<Vec<Rc<RawNode<T>>>> {
        &self.children
    }

    fn print(&self) {
        println!("{:?}", self);
        for i in 0..self.children.borrow().len() {
            RawNode::print(self.children().borrow().get(i).unwrap());
        }
    }

    fn add_child(parent: &Rc<RawNode<T>>, child: &Rc<RawNode<T>>) {
        parent.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }

    pub fn find(self: &Rc<Self>, value: &T) -> Option<Rc<RawNode<T>>> {
        if self.value() == value {
            return Some(self.clone());
        } else {
            for i in 0..self.children.borrow().len() {
                return RawNode::find(self.children().borrow().get(i).unwrap(), &value);
            }
        }
        None
    }
}