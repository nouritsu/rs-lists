use std::mem;
/*
Description :
A singly linked stack that implements push and pop.
Implements Drop trait
*/

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, x: i32) {
        let new_node = Box::new(Node {
            elem: x,
            next: mem::replace(&mut self.head, Link::Empty), //temp swap with Empty
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            //replace elements with empties until you run out of elements
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}

/* TESTS */

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();

        //Empty pop test
        assert_eq!(list.pop(), None);

        //Populate List
        list.push(1);
        list.push(2);
        list.push(3);

        //Non empty pop test
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        //Post pop push test
        list.push(4);
        list.push(5);

        //Pop test
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        //Pop till empty test
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
