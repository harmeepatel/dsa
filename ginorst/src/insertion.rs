pub fn sort(arr: &mut Vec<i32>) {
    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] > arr[i] {
                arr.swap(i, j);
            }
        }
    }
}
