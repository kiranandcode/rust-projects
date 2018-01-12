use std::mem;
use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: None, tail: ptr::null_mut()}
    }
    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None
        });

        let raw_tail: *mut _ = &mut *new_tail;

        // if tail != null, tail->next = new_tail;
        if !self.tail.is_null() {
            // must be derefed as tail is a raw pointer
            unsafe {
            (*self.tail).next = Some(new_tail);
            }
        } else {
            // else self.head = new_tail
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }
}


#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}