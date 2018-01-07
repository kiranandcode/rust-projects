use std::rc::Rc;

pub struct List<T> {
    head: Link<T>
}

// a nullable reference
type Link<T> = Option<Rc<Node<T>>>;

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

    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(
                Rc::new(Node {
                    elem: elem,
                    next: self.head.clone()
                })
            )
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head
            // get a reference to the node or none
            .as_ref()
            // then set to the result optional
            .and_then(|node| node.next.clone())
        }
   }


   pub fn head(&self) -> Option<&T> {
       self.head.as_ref().map(|node| {
           &node.elem
       })
   }
}

pub struct Iter<'a, T:'a> {
    next: Option<&'a Node<T>>
}

impl<T> List<T> {
    // wehn constructing iterators, initialize them with your own lifetime
    pub fn iter<'a>(&'a self) -> Iter<'a,T> {
        Iter {
            next: self.head.as_ref().map(|node| {
               &**node 
            })
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // ** because, node is &Rc<Node<T>>, thus the first * retrieves
            // the Rc, the second one retrieves the internal value
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_list = self.head.take();
        while let Some(node) = cur_list {
            // clone a reference to the current node's next
            cur_list = node.next.clone();
            // drop node, hereby reducing the ref count
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);


        let list = list.tail();
        assert_eq!(list.head(), None);
    }
    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}