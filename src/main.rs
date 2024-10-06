use rust_smartpointers_weak_tree_samples::Node;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

fn main() {
    let leaf = Rc::new(
        Node{
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    );
    let branch = Rc::new(
        Node{
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        }
    );

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
