use std::rc::{Rc, Weak};
use std::fmt::Debug;
use std::cell::RefCell;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Int
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Int(u64)
}

impl Value {
    pub fn get_val(&self) -> u64 {
        let Value::Int(i) = *self;
        i
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Op {
    Add,
    Mult
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    node: T,
    parent: Option<Weak<RefCell<Node<T>>>>,
    lhs: Option<Rc<RefCell<Node<T>>>>,
    rhs: Option<Rc<RefCell<Node<T>>>>
}

impl<T> Node<T> {
    pub fn new(node: T, parent: Option<Weak<RefCell<Node<T>>>>) -> Node<T> {
        Node { node, parent, lhs: None, rhs: None }
    }

    pub fn set_lhs(&mut self, n: Rc<RefCell<Node<T>>>) {
        self.lhs = Some(n);
    }

    pub fn set_rhs(&mut self, n: Rc<RefCell<Node<T>>>) {
        self.rhs = Some(n);
    }

    pub fn clear_lhs(&mut self) {
        self.lhs = None;
    }

    pub fn clear_rhs(&mut self) {
        self.rhs = None;
    }

    pub fn unwrap(self) -> T {
        self.node
    }
}

pub fn try_print_node<T: Debug>(n: &Option<Rc<RefCell<Node<T>>>>) {
    match n {
        Some(r) => {
            print_node(&r.borrow());
        },
        None => print!("NA")
    }
}

pub fn print_node<T: Debug>(n: &Node<T>) {
    print!("{{{:?} L[", n.node);
    try_print_node(&n.lhs);
    print!("] R[");
    try_print_node(&n.rhs);
    print!("]}}");
}
