#![allow(dead_code)]

use std::{
    fmt::{Debug, Display, Formatter, Result},
    ptr::NonNull,
};

fn main() {}

#[derive(Copy, Clone, Debug)]
struct Node<T>
where
    T: Copy,
{
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Display for Node<T>
where
    T: Display + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.next {
            Some(node) => write!(f, "{} {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

impl<T> Node<T>
where
    T: Copy,
{
    fn new(val: T) -> Self {
        Node {
            val,
            next: None,
            prev: None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct DoublyLinkedList<T>
where
    T: Copy,
{
    length: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> Default for DoublyLinkedList<T>
where
    T: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Display for DoublyLinkedList<T>
where
    T: Display + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> DoublyLinkedList<T>
where
    T: Copy,
{
    fn new() -> Self {
        DoublyLinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, val: T) {
        let mut node = Box::new(Node::new(val));
        node.prev = self.tail;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.tail {
            Some(tail) => unsafe { (*tail.as_ptr()).next = node_ptr },
            None => self.head = node_ptr,
        }
        self.tail = node_ptr;
        self.length += 1;
    }

    fn pop(&mut self) -> std::result::Result<T, &str> {
        let node = match self.tail {
            Some(n) => n,
            None => return Err("Tail not found."),
        };
        unsafe {
            self.tail = (*self.tail.unwrap().as_ptr()).prev;
            (*self.tail.unwrap().as_ptr()).next = None;
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = None;
        }
        drop(node);
        self.length -= 1;
        Ok(unsafe { (*node.as_ptr()).val })
    }

    fn push_front(&mut self, val: T) {
        let mut node = Box::new(Node::new(val));
        node.next = self.head;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.head {
            Some(head) => unsafe { (*head.as_ptr()).prev = node_ptr },
            None => self.tail = node_ptr,
        }
        self.head = node_ptr;
        self.length += 1;
    }

    fn pop_front(&mut self) -> std::result::Result<T, &str> {
        let node = match self.head {
            Some(head) => head,
            None => return Err("Head not found"),
        };
        unsafe {
            self.head = (*self.head.unwrap().as_ptr()).next;
            (*self.head.unwrap().as_ptr()).prev = None;
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = None;
        }
        drop(node);
        self.length -= 1;
        Ok(unsafe { (*node.as_ptr()).val })
    }

    fn insert_at() {}

    fn remove_at() {}

    fn replace() {}

    pub fn get(&mut self, index: usize) -> Option<&'static T> {
        Self::get_ith_node(self.head, index)
    }

    fn get_ith_node(node: Option<NonNull<Node<T>>>, index: usize) -> Option<&'static T> {
        match node {
            Some(node_ptr) => match index {
                0 => Some(unsafe { &(*node_ptr.as_ptr()).val }),
                _ => Self::get_ith_node(unsafe { (*node_ptr.as_ptr()).next }, index - 1),
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        match list.get(1) {
            Some(val) => assert_eq!(*val, 2),
            None => panic!("Expected to find {} at index 1", 2),
        }
    }

    #[test]
    fn push_front() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push_front(4);
        match list.get(0) {
            Some(val) => assert_eq!(*val, 4),
            None => panic!("Expected to find {} at index 0", 4),
        }
    }

    #[test]
    fn pop() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        let val = list.pop().unwrap();
        assert_eq!(val, 4);
    }

    #[test]
    fn pop_front() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        let val = list.pop_front().unwrap();
        assert_eq!(val, 1);
    }
}
