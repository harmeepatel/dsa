#![allow(dead_code)]

use std::{borrow::BorrowMut, rc::Rc};

fn main() {}

#[derive(Clone, Debug, Default, PartialEq)]
struct Node<T>
where
// T: std::fmt::Debug,
{
    val: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
// T: std::fmt::Debug,
{
    fn new(val: T) -> Node<T> {
        Node {
            val,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct DLL<T>
where
// T: std::fmt::Debug,
{
    head: Box<Node<T>>,
    length: usize,
}

impl<T> DLL<T>
where
// T: std::fmt::Debug + std::clone::Clone,
{
    fn new(val: T) -> DLL<T> {
        DLL {
            head: Box::new(Node::new(val)),
            length: 1,
        }
    }

    fn push(&mut self, val: T) {
        let _new_node = Rc::new(Node::new(val));
        while let x = self.head.next {
            
        }

        *self.length.borrow_mut() += 1;
    }

    // fn prepend(&mut self, val: T) {
    //     let mut new_node = Node::new(val);
    //     new_node.next = Some(self.head.clone());
    //     // *self.head.borrow_mut() = Rc::new(RefCell::new(new_node));
    //     dbg!(self.head.borrow_mut());
    //     self.length += 1;
    // }

    fn insert_at(self) {}
    fn remove(self) {}
    fn get(self) {}
    fn remove_at(self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_node() {
        let val = 2;
        let nn = DLL::new(val);
        let a = Box::new(Node {
            val,
            next: None,
            prev: None,
        });

        assert_eq!(nn, DLL { head: a, length: 1 });
    }

    // #[test]
    // fn append() {
    //     let val = 2;
    //     let next_val = 4;
    //     let mut nn = DLL::new(val);
    //     nn.append(next_val);
    //     println!("append");
    //     dbg!(&nn);
    //
    //     assert_eq!(nn.length, 2);
    // }
    //
    // #[test]
    // fn prepend() {
    //     let val = 2;
    //     let pre_val = 4;
    //     let mut nn = DLL::new(val);
    //     nn.prepend(pre_val);
    //     println!("prepend");
    //     dbg!(&nn);
    //
    //     assert_eq!(
    //         nn.length,
    //         2 // RefCell::new(Rc::new(Node {
    //           //     val: pre_val,
    //           //     next: None,
    //           //     prev: None,
    //           // }))
    //     );
    // }
}
