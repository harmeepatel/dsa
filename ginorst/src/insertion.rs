pub struct Sort<'a> {
    pub unsorted: &'a mut Vec<isize>,
}

impl Sort<'_> {
    pub fn new(arr: &mut Vec<isize>) -> Sort {
        Sort { unsorted: arr }
    }
    #[allow(dead_code)]
    pub fn sort(&mut self) {
        let arr = &mut self.unsorted;
        for i in 1..arr.len() {
            for j in 0..i {
                if arr[j] > arr[i] {
                    arr.swap(i, j);
                }
            }
        }
    }
    pub fn print(&self) {
        println!("{:?}", &self.unsorted);
    }
}
