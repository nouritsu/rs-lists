use std::mem;

/*
* What did I learn from this code?
* 1) Defining structs, enums, implementation blocks
* 2) Uses of Option enum
* 3) Enums in Rust vs C
* 4) Implementing and uses of the Drop trait
* 5) static function new() returns self paradigmS
* 6) Using basic marcos - unimplemented!() and assert_eq!()
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
