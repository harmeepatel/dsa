mod sorter;

fn main() {
    let mut unsorted = vec![5, 4, 3, 2, 1];
    let mut sorter = sorter::Sorter::new(&mut unsorted);
    sorter.sort();
}
