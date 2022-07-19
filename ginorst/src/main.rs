mod sorter;

fn main() {
    let mut unsorted = vec![
        8, 7, 4, -7, 1, 9, 3, -9, 5, -3, -6, 0, -5, -8, -4, 2, -10, -2, 6, -1, 10,
    ];
    let mut sorter = sorter::Sorter::new(&mut unsorted);
    sorter.sort();
}
