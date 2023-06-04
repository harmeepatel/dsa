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

pub struct Queue<T>
where
    T: Clone,
{
    length: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

pub struct QueueIntoIterator<T>
where
    T: Clone,
{
    q: Queue<T>,
}

impl<T> IntoIterator for Queue<T>
where
    T: Clone,
{
    type Item = T;

    type IntoIter = QueueIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        QueueIntoIterator { q: self }
    }
}

impl<T> Iterator for QueueIntoIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.q.head {
            Some(head) => unsafe {
                let val = (*head.as_ptr()).val.clone();
                self.q.head = (*head.as_ptr()).next;
                Some(val)
            },
            None => None,
        }
    }
}

impl<T> Queue<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Queue {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, val: T) {
        let mut new_node = Box::new(Node::new(val));
        let new_node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node.clone())) });
        self.length += 1;
        match self.tail {
            Some(tail) => {
                new_node.prev = Some(tail);
                unsafe {
                    (*tail.as_ptr()).next = new_node_ptr;
                    self.tail = new_node_ptr;
                }
            }
            None => {
                self.tail = new_node_ptr;
                self.head = new_node_ptr;
            }
        }
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        match self.head {
            Some(head) => unsafe {
                let val = (*head.as_ptr()).val.clone();
                match (*head.as_ptr()).next {
                    Some(next) => {
                        (*head.as_ptr()).next = None;
                        self.head = Some(next);
                        (*self.head.unwrap().as_ptr()).prev = None;
                        Ok(val)
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                        Ok(val)
                    }
                }
            },
            None => Err("Queue is Empty!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut s = Queue::<i32>::new();
        s.enqueue(1);
        s.enqueue(2);
        s.enqueue(3);
        assert_eq!(s.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn pop() {
        let mut s = Queue::<i32>::new();
        let mut a: Vec<i32> = vec![];
        s.enqueue(1);
        s.enqueue(2);
        s.enqueue(3);
        a.push(s.dequeue().unwrap());
        a.push(s.dequeue().unwrap());
        a.push(s.dequeue().unwrap());
        assert_eq!(a, vec![1, 2, 3]);
    }

    #[test]
    fn underflow() {
        let mut s = Queue::<i32>::new();
        assert_eq!(s.dequeue().err().unwrap(), "Queue is Empty!");
    }

}
