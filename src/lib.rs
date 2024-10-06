use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug)]
pub struct Node<T>{
    pub value: T,
    pub parent: RefCell<Weak<Node<T>>>,
    pub children: RefCell<Vec<Rc<Node<T>>>>,
}