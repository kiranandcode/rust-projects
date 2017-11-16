use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell; 
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::*;


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
//    let list = Cons(1,
 //                   Box::new(Cons(2,
  //                                Box::new(Cons(3,
   //                                        Box::new(Nil))))));

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![leaf.clone()])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());

}
