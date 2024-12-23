#[derive(Debug)]
pub struct Stack<T, const SIZE: usize> {
    elems: [Option<T>; SIZE],
    head: usize,
    capacity: usize,
}

impl<T, const SIZE: usize> Stack<T, SIZE>
where
    T: Copy,
{
    pub fn new() -> Self {
        Stack {
            elems: [None; SIZE],
            head: 0,
            capacity: SIZE,
        }
    }

    pub fn push(&mut self, val: T) -> Result<(), &str> {
        if self.capacity == 0 {
            return Err("Stack capacity reached!");
        }
        match self.elems[self.head] {
            Some(_) => {
                if self.head == self.capacity - 1 {
                    return Err("Stack capacity reached!");
                }
                self.head += 1;
                self.elems[self.head] = Some(val);
                Ok(())
            }
            None => {
                self.elems[self.head] = Some(val);
                Ok(())
            }
        }
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.capacity == 0 {
            return Err("Stack is Empyt!");
        }
        match self.elems[self.head] {
            Some(val) => {
                self.elems[self.head] = None;
                if self.head > 0 {
                    self.head -= 1;
                }
                Ok(val)
            }
            None => Err("Stack is Empyt!"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn push() {
        let mut s = Stack::<i32, 32>::new();
        let mut a: [Option<i32>; 32] = [None; 32];

        s.push(1).ok();
        a[0] = Some(1);
        s.push(2).ok();
        a[1] = Some(2);
        s.push(3).ok();
        a[2] = Some(3);

        assert_eq!(a, s.elems);
    }

    #[test]
    fn pop() {
        let mut s = Stack::<i32, 32>::new();
        let mut a: [Option<i32>; 32] = [None; 32];

        s.push(1).ok();
        a[0] = Some(1);
        s.push(2).ok();
        a[1] = Some(2);
        s.push(3).ok();

        let p: i32 = match s.pop() {
            Ok(val) => val,
            Err(e) => {
                dbg!(e);
                return;
            }
        };

        s.push(4).ok();
        a[2] = Some(4);

        assert_eq!(p, 3);
        assert_eq!(a, s.elems);

        let p: i32 = match s.pop() {
            Ok(val) => val,
            Err(e) => {
                dbg!(e);
                return;
            }
        };
        assert_eq!(p, 4);

        let p: i32 = match s.pop() {
            Ok(val) => val,
            Err(e) => {
                dbg!(e);
                return;
            }
        };
        assert_eq!(p, 2);

        let p: i32 = match s.pop() {
            Ok(val) => val,
            Err(e) => {
                dbg!(e);
                return;
            }
        };
        assert_eq!(p, 1);

        let a: [Option<i32>; 32] = [None; 32];
        assert_eq!(a, s.elems);
    }

    #[test]
    fn overflow() {
        let mut s = Stack::<i32, 0>::new();
        match s.push(1) {
            Ok(_) => unreachable!(),
            Err(e) => assert_eq!(e, "Stack capacity reached!"),
        };
    }

    #[test]
    fn is_empty() {
        let mut s = Stack::<i32, 0>::new();
        match s.pop() {
            Ok(_) => unreachable!(),
            Err(e) => assert_eq!(e, "Stack is Empyt!"),
        };
    }
}
