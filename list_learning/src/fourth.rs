use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

// reference counted pointer to a box that can be borrowed
// rc is a box that can be shared but only viewed through refs.
// refcell allows mutation through a ref
// rc<refcell> produces the equivalent of a box, but shareable
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>
}

impl <T> Node<T> {
    // new returns a shareable box containing itself
    fn new(elem: T) -> Rc<RefCell<Self>> {
        // rc is a ref counted box
        Rc::new(
            // refcell allows interior mutability
            RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None
        }))
    }
    // rc<refcell> is now equivalent to a pointer.
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None, // optional<rc<refcell>> is like a nullable pointer
            tail: None  // optional<rc<refcell>> is like a nullable pointer
        }
    }

    pub fn push_front(&mut self, elem: T) {
        // construct a shareable box for the element
        // void *new_head = node_new(T);
        let new_head = Node::new(elem);
        // void *match = list->head;
        // list->head = NULL;
        match self.head.take() {
            // if(match != NULL)
            Some(old_head)  => {
                //  (*match) // done by borrow_mut
                //          .prev          = new_head;
                //  (*new_head).next       = match;
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 ref of new_head
                new_head.borrow_mut().next = Some(old_head);         // +1 old_head
                // list->head = new_head;
                self.head = Some(new_head);             // +1 new_head, -1 old_head
            }
            None => {
                // list->tail = new_head;
                self.tail = Some(new_head.clone());     // +1 new_head
                // list->head = new_head;
                self.head = Some(new_head);             // +1 new_head
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            // old_head is a Rc<Refcell<>>
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // clear the prev of the new head
                    new_head.borrow_mut().prev.take();
                    // set the head to new_head
                    self.head = Some(new_head);
                },
                None => {
                    // next is none means this is the only element, hence
                    // the list also has a reference to it
                    // take the reference
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        }) 
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }


    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}


#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
    }
}