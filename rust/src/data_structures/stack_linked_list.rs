use std::ptr::NonNull;

#[derive(Copy, Clone)]
struct Node<T>
where
    T: Clone,
{
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T>
where
    T: Clone,
{
    fn new(val: T) -> Self {
        Node {
            val,
            next: None,
            prev: None,
        }
    }
}

pub struct LLStack<T>
where
    T: Clone,
{
    length: usize,
    head: Option<NonNull<Node<T>>>,
}

pub struct LLStackIntoIterator<T>
where
    T: Clone,
{
    lls: LLStack<T>,
}

impl<T> IntoIterator for LLStack<T>
where
    T: Clone,
{
    type Item = T;

    type IntoIter = LLStackIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LLStackIntoIterator { lls: self }
    }
}

// should this be reversed?
// pushing: 1, 2, 3
// currently returns: [3,2,1]
// should return?: [1,2,3]
impl<T> Iterator for LLStackIntoIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lls.head {
            Some(head) => unsafe {
                let val = (*head.as_ptr()).val.clone();
                self.lls.head = (*head.as_ptr()).prev;
                Some(val)
            },
            None => None,
        }
    }
}

impl<T> LLStack<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        LLStack {
            length: 0,
            head: None,
        }
    }

    pub fn push(&mut self, val: T) {
        let mut new_node = Box::new(Node::new(val));
        match self.head {
            Some(head) => {
                new_node.prev = self.head;
                unsafe {
                    (*head.as_ptr()).next =
                        Some(NonNull::new_unchecked(Box::into_raw(new_node.clone())));
                    self.head = Some(NonNull::new_unchecked(Box::into_raw(new_node.clone())));
                }
                self.length += 1;
            }
            None => {
                self.head = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
                self.length += 1;
            }
        }
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        match self.head {
            Some(head) => unsafe {
                match (*head.as_ref()).prev {
                    Some(prev) => {
                        let val = (*head.as_ptr()).val.clone();
                        (*head.as_ptr()).prev = None;
                        self.head = Some(prev);
                        (*self.head.unwrap().as_ptr()).next = None;
                        self.length -= 1;
                        Ok(val)
                    }
                    None => {
                        let val = (*head.as_ptr()).val.clone();
                        self.head = None;
                        self.length -= 1;
                        Ok(val)
                    }
                }
            },
            None => Err("Stack is Empty!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut s = LLStack::<i32>::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.into_iter().collect::<Vec<_>>(), vec![3, 2, 1]);
    }

    #[test]
    fn pop() {
        let mut s = LLStack::<i32>::new();
        let mut a: Vec<i32> = vec![];
        s.push(1);
        s.push(2);
        s.push(3);
        a.push(s.pop().unwrap());
        a.push(s.pop().unwrap());
        a.push(s.pop().unwrap());
        assert_eq!(a, vec![3, 2, 1]);
    }

    #[test]
    fn underflow() {
        let mut s = LLStack::<i32>::new();
        assert_eq!(s.pop().err().unwrap(), "Stack is Empty!");
    }
}
