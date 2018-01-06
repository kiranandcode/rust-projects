use std::mem;

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}


impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next:  self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
                let node = *node;
                self.head = node.next;
                node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        /*
        // This is what we want
            #define T int
            struct node {
                T elem;
                struct node *next;
            }
            T *peek(struct list *list) {
                if(list->head != NULL) {
                    return &list->head->elem
                }
                return NULL;
            }
        // map takes the value, so self.head.map() would be
        T *peek(struct list *list) {
            if(list->head != NULL) {
                struct node *node = list->head;
                list->head = NULL;
                return &node->elem;
            }
            return NULL;
        }

        // as_ref takes a copy of the node so would be
        T *peek(struct list *list) {
            if(list->head != NULL) {
                struct node *node = list->head;
                return &node->elem;
            }
            return NULL;
        }


        // note we also return &T so that we don't have to copy the object - if we didn't the value would be moved
        // for a type like i32, a return would be fine as it implements copy,
        // but for more complex things we would want to use a reference
        
        T peek(struct list *list) {
            if(list->head != NULL) {
                struct node *node = list->head;
                T result = node->elem;
                node->elem =  NULL;
                return result;
            }
            return NULL;
        }

        */
        self.head.as_ref().map(|node| {
            // we need to return a reference to the node, hence 
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_node = self.head.take();

        while let Some(mut boxed_node) = current_node {
            current_node = boxed_node.next.take();
        }
    }
}


pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }


    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }
}