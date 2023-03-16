fn main() {}

#[allow(dead_code)]
fn sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        for j in 0..arr.len() - i {
            if arr[j] > arr[j + 1] {
                let a = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = a;
            }
        }
    }
}

#[allow(dead_code)]
fn rev_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] > arr[j + 1] {
                let a = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = a;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_end_to_begining() {
        let mut a = [6, 36, 7, 2, 5, -23, 7, 323, 123, 345];
        let mut b = a.clone();

        b.sort();
        sort(&mut a);
        assert_eq!(a, b);
    }

    #[test]
    fn bubble_sort_begining_to_end() {
        let mut a = [6, 36, 7, 2, 5, -23, 7, 323, 123, 345];
        let mut b = a.clone();

        b.sort();
        rev_sort(&mut a);
        assert_eq!(a, b);
    }
}
