#![allow(dead_code)]

use std::{dbg, ptr::NonNull};

fn main() {}

#[derive(Copy, Clone)]
struct Node<T>
where
    T: Copy,
{
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
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
#[derive(Copy, Clone)]
struct DoublyLinkedList<T>
where
    T: Copy,
{
    length: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

#[derive(Copy, Clone)]
struct DoublyLinkedListIntoIterator<T>
where
    T: Copy,
{
    dll: DoublyLinkedList<T>,
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

impl<T> Iterator for DoublyLinkedListIntoIterator<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dll.head {
            Some(head) => unsafe {
                let val = (*head.as_ptr()).val;
                self.dll.head = (*head.as_ptr()).next;
                Some(val)
            },
            None => None,
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
        let mut new_node = Box::new(Node::new(val));
        new_node.prev = self.tail;
        let new_node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
        match self.tail {
            Some(tail) => unsafe {
                (*tail.as_ptr()).next = new_node_ptr;
            },
            None => self.head = new_node_ptr,
        }
        self.tail = new_node_ptr;
        self.length += 1;
    }

    fn pop(&mut self) -> Option<T>
    where
        T: Copy,
    {
        let pop_node = self.tail;
        match pop_node {
            Some(node) => unsafe {
                match (*node.as_ptr()).prev {
                    Some(prev) => {
                        self.tail = Some(prev);
                        (*self.tail.unwrap().as_ptr()).next = None;
                        (*node.as_ptr()).prev = None;
                    }
                    None => {
                        self.tail = None;
                        self.head = None;
                    }
                }
            },
            None => return None,
        }
        self.length -= 1;
        Some(unsafe { (*pop_node.unwrap().as_ptr()).val })
    }

    fn push_front(&mut self, val: T) {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = self.head;
        let new_node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
        match self.head {
            Some(head) => unsafe {
                (*head.as_ptr()).prev = new_node_ptr;
            },
            None => self.tail = new_node_ptr,
        }
        self.head = new_node_ptr;
        self.length += 1;
    }

    fn pop_front(&mut self) -> Option<T>
    where
        T: Copy,
    {
        let pop_node = self.head;
        match pop_node {
            Some(node) => unsafe {
                match (*node.as_ptr()).next {
                    Some(next) => {
                        self.head = Some(next);
                        (*self.head.unwrap().as_ptr()).prev = None;
                        (*node.as_ptr()).next = None;
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }
            },
            None => return None,
        }

        self.length -= 1;
        Some(unsafe { (*pop_node.unwrap().as_ptr()).val })
    }

    pub fn insert_at(&mut self, index: usize, val: T) {
        if index > self.length {
            dbg!("Index out of range!");
            return;
        }
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
        }
        self.length += 1;
    }

    fn insert_at_ith(node: Option<NonNull<Node<T>>>, val: T, index: usize, start_from_head: bool) {
        match node {
            Some(current_node) => match index {
                0 => {
                    let mut new_node = Box::new(Node::new(val));
                    new_node.next = Some(current_node);
                    unsafe {
                        new_node.prev = (*current_node.as_ptr()).prev;
                        if let Some(prev) = (*current_node.as_ptr()).prev {
                            (*prev.as_ptr()).next =
                                Some(NonNull::new_unchecked(Box::into_raw(new_node.clone())));
                        } else {
                            dbg!("Previous Node does not exist.");
                        }
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
            None => {
                dbg!("Current Node is EMPTY!");
            }
        }
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        if index >= self.length {
            dbg!("Index out of range.");
            return None;
        }
        if index == 0 {
            return self.pop_front();
        }
        if index == self.length - 1 {
            return self.pop();
        }

        let start_from_head = index < self.length / 2;
        match match start_from_head {
            true => Self::remove_at_ith(self.head, index, start_from_head),
            false => Self::remove_at_ith(self.tail, self.length - 1 - index, start_from_head),
        } {
            Some(node) => {
                self.length -= 1;
                return unsafe { Some((*node.as_ptr()).val) };
            }
            None => None,
        }
    }

    fn remove_at_ith(
        node: Option<NonNull<Node<T>>>,
        index: usize,
        start_from_head: bool,
    ) -> Option<NonNull<Node<T>>> {
        match node {
            Some(current_node) => match index {
                0 => unsafe {
                    if let Some(next) = (*current_node.as_ptr()).next {
                        (*next.as_ptr()).prev = (*current_node.as_ptr()).prev;
                    }
                    if let Some(prev) = (*current_node.as_ptr()).prev {
                        (*prev.as_ptr()).next = (*current_node.as_ptr()).next;
                    }
                    (*current_node.as_ptr()).next = None;
                    (*current_node.as_ptr()).prev = None;
                    Some(current_node)
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

            None => {
                dbg!("Current Node is EMPTY!");
                None
            }
        }
    }

    pub fn replace(&mut self, index: usize, val: T) -> Option<T> {
        if index >= self.length {
            dbg!("Index out of range.");
            return None;
        }

        let start_from_head = index < self.length / 2;
        match start_from_head {
            true => Self::replace_ith(self.head, val, index, start_from_head),
            false => Self::replace_ith(self.tail, val, self.length - 1 - index, start_from_head),
        }
    }

    fn replace_ith(
        node: Option<NonNull<Node<T>>>,
        val: T,
        index: usize,
        start_from_head: bool,
    ) -> Option<T> {
        match node {
            Some(current_node) => match index {
                0 => unsafe {
                    let old_val = (*current_node.as_ptr()).val;
                    (*current_node.as_ptr()).val = val;
                    Some(old_val)
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

            None => {
                dbg!("Current Node is EMPTY!");
                None
            }
        }
    }

    pub fn get(&mut self, index: usize) -> Option<&'static T> {
        let start_from_head = index < self.length / 2;
        match start_from_head {
            true => Self::get_ith_node(self.head, index, start_from_head),
            false => Self::get_ith_node(self.tail, self.length - 1 - index, start_from_head),
        }
    }

    fn get_ith_node<'a>(
        node: Option<NonNull<Node<T>>>,
        index: usize,
        start_from_head: bool,
    ) -> Option<&'a T> {
        match node {
            Some(node) => match index {
                0 => Some(unsafe { &(*node.as_ptr()).val }),
                _ => Self::get_ith_node(
                    unsafe {
                        if start_from_head {
                            (*node.as_ptr()).next
                        } else {
                            (*node.as_ptr()).prev
                        }
                    },
                    index - 1,
                    start_from_head,
                ),
            },
            None => todo!(),
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
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(arr, vec![1, 2, 3]);
    }

    #[test]
    fn push_front() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push(1);
        list.push_front(4);
        list.push(2);
        list.push_front(3);
        list.push(5);
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(arr, vec![3, 4, 1, 2, 5]);
    }

    #[test]
    fn pop() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        match list.pop() {
            Some(val) => {
                assert_eq!(val, 3);
            }
            None => {
                dbg!("First pop failed.");
            }
        };
        match list.pop() {
            Some(val) => {
                assert_eq!(val, 2);
            }
            None => {
                dbg!("Second pop failed.");
            }
        };
        match list.pop() {
            Some(val) => {
                assert_eq!(val, 1);
            }
            None => {
                dbg!("Third pop failed.");
            }
        };
        match list.pop() {
            Some(_) => {
                dbg!("Should not have found this.");
            }
            None => {
                assert!(true);
            }
        };
    }

    #[test]
    fn pop_front() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        match list.pop_front() {
            Some(val) => {
                assert_eq!(val, 1);
            }
            None => {
                dbg!("First pop_front failed.");
            }
        };
        match list.pop_front() {
            Some(val) => {
                assert_eq!(val, 2);
            }
            None => {
                dbg!("Second pop_front failed.");
            }
        };
        match list.pop_front() {
            Some(val) => {
                assert_eq!(val, 3);
            }
            None => {
                dbg!("Third pop_front failed.");
            }
        };
        match list.pop_front() {
            Some(_) => {
                dbg!("Should not have found this.");
            }
            None => {
                assert!(true);
            }
        };
    }

    #[test]
    fn insert_at() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.insert_at(0, 4);
        let mut arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(arr, vec![4, 1, 2, 3]);

        list.insert_at(2, 5);
        arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(arr, vec![4, 1, 5, 2, 3]);

        list.insert_at(4, 6);
        arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(arr, vec![4, 1, 5, 2, 6, 3]);

        list.insert_at(list.length, 7);
        arr = list.into_iter().collect::<Vec<_>>();
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

        let mut v = list.remove_at(0).unwrap();
        let mut arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, 1);
        assert_eq!(arr, vec![2, 3, 4, 5, 6]);

        v = list.remove_at(1).unwrap();
        arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, 3);
        assert_eq!(arr, vec![2, 4, 5, 6]);

        v = list.remove_at(list.length - 1).unwrap();
        arr = list.into_iter().collect::<Vec<_>>();
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

        let v = list.replace(0, 10).unwrap();
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, 1);
        assert_eq!(arr, vec![10, 2, 3, 4, 5]);

        let v = list.replace(2, 30).unwrap();
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, 3);
        assert_eq!(arr, vec![10, 2, 30, 4, 5]);

        let v = list.replace(list.length - 1, 50).unwrap();
        let arr = list.into_iter().collect::<Vec<_>>();
        assert_eq!(v, 5);
        assert_eq!(arr, vec![10, 2, 30, 4, 50]);
    }
}
