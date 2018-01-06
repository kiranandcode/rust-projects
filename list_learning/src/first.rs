use std::mem;


pub struct List {
    // the first pointer is a link to allow lists of 0 size
    head: Link
}

// link is just a wrapper around a node
enum Link {
    Empty,
    // hence it wouldn't make sense to put a Box<Link> here.
    More(Box<Node>)
}

struct Node {
    elem: i32,
    // we reference back up the type heirarchy
    // this seems bad, but when you think of link as a wrapper around a pointer to Node,
    // it makes more sense
    // the fact that a node pointer may be null is encapsulated in this
    next: Link 
}


/*

struct List {
    struct Link *head;
}

// essentially this, we treat as if zero when node == null
struct Link { 
    struct Node *node;
}

struct Node {
    int elem;
    struct Link *next;
}

// but due to compile time optimizations,
type struct Link = struct Node *;
// so the whole thing becomes

struct List {
    struct Node *head;
}

struct Node {
    int elem;
    struct Node *next;
}

// the whole link thing is gone, but during compilation it was used to type secure the ability
// to null a pointer and treat that as zero

*/

impl List {
    pub fn new() -> Self {
        List {
            // return a list with a null pointer for head - it has size zero
            head: Link::Empty
        }
    }

    pub fn push(&mut self, elem: i32) {
        // how to push into a std C linked list
        // create node element, 
        // place elem into node,
        // set next pointer to head of list
        // set head pointer of list struct to new elem

        // new node in c would be a pointer to the node
        let new_node = Box::new(Node {
            elem: elem,
            // we have to do this in a function - probably using unsafe inside,
            // as to swap values, there is an intermediate step in which the variables
            // enter an undefined state
            // i.e
            // // swap x,y
            // temp = x
            // x = y
            // -----------------------> undefined point, both x and y = y
            // y = temp
            // hence to do this swap, and ensure at all points everyone is happy,
            // we do mem::replace
            next: mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // how to pop from c linked list
        // check list struct pointer for null
        // retrieve  head node
        // set list struct pointer to head->next
        // return node elem

        // this is essentially, 
        // let temp = list.head;
        // list.head = NULL;
        // if(temp == NULL) {
        //    return NULL
        // } else {
        //   list.head = temp.next; 
        //   void *value = temp.value;
        //   free(temp);
        //   return value;
        // }
        match mem::replace(&mut self.head, Link::Empty) {
            // check list struct pointer for null
            Link::Empty => None,
            // retrieve head node
            Link::More(boxed_node) => {
                   // at the moment boxed_node is a pointer to a node 
                   // the assignment of self.head, is a move operation
                   // if we were to move from boxed_node,
                   // this would leave boxed_node in an invalid state, thus
                   // boxed_node is consumed
                   // to avoid this, we want to get the whole node
                   // once this is on the stack, rust can allow us to retrieve internal values 
                   // as though they were variables
                   // this is a move out of box operation
                   // equivalent to
                   // struct node *boxed_node= some_pointer;
                   // struct node node;
                   // node.elem = boxed_node->elem;
                   // node.next = boxed_node->next;
                   // free(boxed_node);
                   let node = *boxed_node;
                   // move the node.next variable out, 
                   self.head = node.next;
                   //at this point node.next is not touchable, but the node still is
                   // use this fact to return the other field in the node
                   Some(node.elem) 
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // first retrieve the nullable pointer from the list struct
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        // iterative list deletion
        // while((boxed_node != null)) {
        //     struct node *current_node = boxed_node;
        //  // the following two lines are achieved using the mem::replace
        //     boxed_node = boxed_node->next;
        //     current_node->next = null;
        //  // the free occurs automatically by the rust compiler
        //    free(boxed_node);
        // }
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }

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
}