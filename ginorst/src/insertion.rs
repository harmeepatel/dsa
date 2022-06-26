struct Insertion<'a> {
    unsorted: &'a mut Vec<isize>,
}

impl Insertion<'_> {
    fn new(arr: &mut Vec<isize>) -> Insertion {
        Insertion { unsorted: arr }
    }
    fn sort(&mut self) {
        let arr = &mut self.unsorted;
        for i in 1..arr.len() {
            for j in 0..i {
                if arr[j] > arr[i] {
                    arr.swap(i, j);
                }
            }
        }
    }
}

pub fn sort(arr: &mut Vec<i32>) {}
