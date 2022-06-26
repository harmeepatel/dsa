mod insertion;

fn main() {
    let mut unsorted = vec![5, 4, 3, 2, 1];
    let mut sorter = insertion::Sort::new(&mut unsorted);
    sorter.sort();
}
