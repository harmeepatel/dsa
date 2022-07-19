mod insertion;

pub struct Sorter<'a> {
    pub unsorted: &'a mut Vec<isize>,
}

impl Sorter<'_> {
    pub fn new(arr: &mut Vec<isize>) -> Sorter {
        Sorter { unsorted: arr }
    }
    pub fn sort(&mut self) {
        let mut arr = &mut self.unsorted;
        println!("unsorted:\t{:?}", &arr);
        insertion::sort(&mut arr);
        println!("sorted: \t{:?}", &arr);
    }
}
