fn main() {}

#[allow(dead_code)]
fn linear_search(arr: &[i32], q: &i32) -> bool {
    for i in arr {
        if i == q {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_search_element() {
        let a = [6, 36, 7, 2, 5, -23, 7, 323, 123, 345];

        assert!(linear_search(&a, &123));
    }
}
