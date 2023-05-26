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
struct DoublyLinkedListIntoIterator<T>
where
    T: Copy,
{
    dll: DoublyLinkedList<T>,
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

impl<T> Iterator for DoublyLinkedListIntoIterator<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dll.head == None {
            return None;
        } else {
            unsafe {
                let val = (*self.dll.head.unwrap().as_ptr()).val;
                self.dll.head = (*self.dll.head.unwrap().as_ptr()).next;
                Some(val)
            }
        }
    }
}

impl<T> IntoIterator for DoublyLinkedList<T>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = DoublyLinkedListIntoIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        DoublyLinkedListIntoIterator { dll: self }
    }
}

impl<T> DoublyLinkedList<T>
where
    T: Display + Copy,
{
    fn new() -> Self {
        DoublyLinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, val: T) {
        let mut new_node = Box::new(Node::new(val));
        new_node.prev = self.tail;
        let new_node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
        match self.tail {
            Some(tail) => unsafe { (*tail.as_ptr()).next = new_node_ptr },
            None => self.head = new_node_ptr,
        }
        self.tail = new_node_ptr;
        self.length += 1;
    }

    fn pop(&mut self) -> std::result::Result<T, String> {
        let node = match self.tail {
            Some(n) => n,
            None => return Err("Tail not found.".to_owned()),
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
        let mut new_node = Box::new(Node::new(val));
        new_node.next = self.head;
        let new_node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
        match self.head {
            Some(head) => unsafe { (*head.as_ptr()).prev = new_node_ptr },
            None => self.tail = new_node_ptr,
        }
        self.head = new_node_ptr;
        self.length += 1;
    }

    fn pop_front(&mut self) -> std::result::Result<T, String> {
        let node = match self.head {
            Some(head) => head,
            None => return Err("Head not found".to_owned()),
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

    pub fn insert_at(&mut self, val: T, index: usize) {
        if index == 0 {
            self.push_front(val);
            return;
        }
        if index == self.length {
            self.push(val);
            return;
        }

        let start_from_head = index < self.length / 2;
        match start_from_head {
            true => Self::insert_at_ith(self.head, val, index, start_from_head),
            false => Self::insert_at_ith(self.tail, val, self.length - 1 - index, start_from_head),
        };
        self.length += 1;
    }

    fn insert_at_ith(node: Option<NonNull<Node<T>>>, val: T, index: usize, start_from_head: bool) {
        match node {
            Some(current_node) => match index {
                0 => {
                    let mut new_node = Box::new(Node::new(val));
                    new_node.next = Some(current_node);
                    unsafe {
                        new_node.prev = (*current_node.as_ref()).prev;
                        (*(*current_node.as_ptr()).prev.unwrap().as_ptr()).next =
                            Some(NonNull::new_unchecked(Box::into_raw(new_node.clone())));
                        (*current_node.as_ptr()).prev =
                            Some(NonNull::new_unchecked(Box::into_raw(new_node.clone())));
                    }
                }
                _ => Self::insert_at_ith(
                    unsafe {
                        if start_from_head {
                            (*current_node.as_ptr()).next
                        } else {
                            (*current_node.as_ptr()).prev
                        }
                    },
                    val,
                    index - 1,
                    start_from_head,
                ),
            },
            None => println!("Node not found"),
        }
    }

    pub fn remove_at(&mut self, index: usize) -> std::result::Result<T, String> {
        if index >= self.length {
            return Err(
                format!("Index out of range.\nlinked list length: {}", self.length),
            );
        }

        if index == 0 {
            return self.pop_front();
        }
        if index == self.length - 1 {
            return self.pop();
        }

        let start_from_head = index < self.length / 2;
        match start_from_head {
            true => {
                let val = Self::remove_at_ith(self.head, index, start_from_head);
                self.length -= 1;
                return val;
            }
            false => {
                let val = Self::remove_at_ith(self.tail, self.length - 1 - index, start_from_head);
                self.length -= 1;
                return val;
            }
        };
    }

    fn remove_at_ith(
        node: Option<NonNull<Node<T>>>,
        index: usize,
        start_from_head: bool,
    ) -> std::result::Result<T, String> {
        match node {
            Some(current_node) => match index {
                0 => unsafe {
                    (*(*current_node.as_ptr()).prev.unwrap().as_ptr()).next =
                        (*current_node.as_ptr()).next;
                    (*(*current_node.as_ptr()).next.unwrap().as_ptr()).prev =
                        (*current_node.as_ptr()).prev;

                    (*current_node.as_ptr()).next = None;
                    (*current_node.as_ptr()).prev = None;

                    Ok((*current_node.as_ptr()).val)
                },
                _ => Self::remove_at_ith(
                    unsafe {
                        if start_from_head {
                            (*current_node.as_ptr()).next
                        } else {
                            (*current_node.as_ptr()).prev
                        }
                    },
                    index - 1,
                    start_from_head,
                ),
            },
            None => Err("Node not found.".to_owned()),
        }
    }

    pub fn replace(&mut self, val: T, index: usize) -> std::result::Result<T, String> {
        if index >= self.length {
            return Err(format!(
                "Index out of range.\nlinked list length: {}",
                self.length
            ));
        }
        let start_from_head = index < self.length / 2;

        match start_from_head {
            true => Self::replace_ith(self.head, val, index, start_from_head),
            false => Self::replace_ith(self.tail, val, self.length - 1 - index, start_from_head),
        }
    }
    fn replace_ith<'a>(
        node: Option<NonNull<Node<T>>>,
        val: T,
        index: usize,
        start_from_head: bool,
    ) -> std::result::Result<T, String> {
        match node {
            Some(current_node) => match index {
                0 => unsafe {
                    let curr_val = (*current_node.as_ptr()).val;
                    (*current_node.as_ptr()).val = val;
                    return Ok(curr_val);
                },

                _ => Self::replace_ith(
                    unsafe {
                        if start_from_head {
                            (*current_node.as_ptr()).next
                        } else {
                            (*current_node.as_ptr()).prev
                        }
                    },
                    val,
                    index - 1,
                    start_from_head,
                ),
            },
            None => Err("Node not found.".to_owned()),
        }
    }

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

    #[test]
    fn insert_at() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.insert_at(4, 0);
        list.insert_at(5, 2);
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(arr, vec![4, 1, 5, 2, 3]);

        list.insert_at(6, 4);
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(arr, vec![4, 1, 5, 2, 6, 3]);

        list.insert_at(7, list.length);
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(arr, vec![4, 1, 5, 2, 6, 3, 7]);
    }

    #[test]
    fn remove_at() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.push(6);

        let v = list.remove_at(0).unwrap();
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, 1);
        assert_eq!(arr, vec![2, 3, 4, 5, 6]);

        let v = list.remove_at(1).unwrap();
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, 3);
        assert_eq!(arr, vec![2, 4, 5, 6]);

        let v = list.remove_at(list.length - 1).unwrap();
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, 6);
        assert_eq!(arr, vec![2, 4, 5]);
    }

    #[test]
    fn replace() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);

        let v = list.replace(10, 0).unwrap();
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, 1);
        assert_eq!(arr, vec![10, 2, 3, 4, 5]);

        let v = list.replace(30, 2).unwrap();
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, 3);
        assert_eq!(arr, vec![10, 2, 30, 4, 5]);

        let v = list.replace(50, list.length - 1).unwrap();
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, 5);
        assert_eq!(arr, vec![10, 2, 30, 4, 50]);
    }
}
