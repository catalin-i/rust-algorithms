use std::cmp::{PartialEq, PartialOrd};

pub fn merge_sorted_arrays<T: PartialEq + PartialOrd + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    let mut left_side = a.iter();
    let mut right_side = b.iter();
    let mut result: Vec<T> = Vec::new();

    let mut finished_left = false;
    let mut finished_right = false;

    let mut left = left_side.next();
    let mut right = right_side.next();

    loop {
        if left == None && right == None {
            break;
        }

        if !finished_left && finished_right || left <= right {
            match left {
                Some(num) => {
                    result.push(*num);
                    left = left_side.next();
                }
                None => finished_left = true,
            }
        }
        if !finished_right && finished_left || left > right {
            match right {
                Some(num) => {
                    result.push(*num);
                    right = right_side.next();
                }
                None => finished_right = true,
            }
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use crate::algorithms::sorting::merge_sorted_arrays;

    #[test]
    fn typical_use_case() {
        let a = [12, 20, 65, 100, 120] as [i32; 5];
        let b = [1, 2, 32, 46, 64, 78, 90] as [i32; 7];
        let result = vec![1, 2, 12, 20, 32, 46, 64, 65, 78, 90, 100, 120];
        assert_eq!(merge_sorted_arrays(&a, &b), result);
    }
    #[test]
    fn merge_two_empty() {
        let a: [i32; 0] = [];
        let b: [i32; 0] = [];
        let result = vec![];
        assert_eq!(merge_sorted_arrays(&a, &b), result);
    }

    #[test]
    fn merge_one_empty() {
        let a: [i32; 1] = [54];
        let b: [i32; 0] = [];
        let result = vec![54];
        assert_eq!(merge_sorted_arrays(&a, &b), result);
    }

    #[test]
    fn merge_string_arrays() {
        let a: [&str; 2] = ["apples", "oranges"];
        let b: [&str; 1] = ["bananas"];
        let result = vec!["apples", "bananas", "oranges"];
        assert_eq!(merge_sorted_arrays(&a, &b), result);
    }

    #[test]
    fn merge_char_arrays() {
        let a: [char; 2] = ['a', 'o'];
        let b: [char; 1] = ['b'];
        let result = vec!['a', 'b', 'o'];
        assert_eq!(merge_sorted_arrays(&a, &b), result);
    }
}
