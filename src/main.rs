pub mod lexer;

use lexer::Node;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let root = Rc::new(RefCell::new(Node::new(10, None)));
    let root_weak = Rc::downgrade(&root);
    let n1 = Node::new(11, Some(root_weak));
    let n1_ref = Rc::new(RefCell::new(n1));
    root.borrow_mut().set_lhs(n1_ref);
    lexer::print_node(&root.borrow());
    println!("");
}
