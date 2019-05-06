pub mod lexer;

use lexer::Node;
use lexer::Value;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let root = Rc::new(RefCell::new(Node::new(Value::Int(1), None)));
    let root_weak = Rc::downgrade(&root);
    let n1 = Node::new(Value::Int(2), Some(root_weak));
    let n1_ref = Rc::new(RefCell::new(n1));
    root.borrow_mut().set_lhs(n1_ref);
    lexer::print_node(&root.borrow());
    println!("");
}
