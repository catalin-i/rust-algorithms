#![allow(dead_code)]
mod algorithms;
mod exercises;
#[cfg(test)]
mod tests_sort {
    use crate::algorithms::sorting::merge_sort;

    #[test]
    fn typical_use_case() {
        let a = [12, 20, 65, 100, 120] as [i32; 5];
        let b = [1, 2, 32, 46, 64, 78, 90] as [i32; 7];
        let result = vec![1, 2, 12, 20, 32, 46, 64, 65, 78, 90, 100, 120];
        assert_eq!(merge_sort(&a, &b), result);
    }
    #[test]
    fn merge_two_empty() {
        let a: [i32; 0] = [];
        let b: [i32; 0] = [];
        let result = vec![];
        assert_eq!(merge_sort(&a, &b), result);
    }

    #[test]
    fn merge_one_empty() {
        let a: [i32; 1] = [54];
        let b: [i32; 0] = [];
        let result = vec![54];
        assert_eq!(merge_sort(&a, &b), result);
    }
}
#[cfg(test)]
mod tests_exercises {
    use crate::exercises::first_repeating_token::find_repeat;

    #[test]
    fn found_int_result() {
        let input: [i32; 6] = [12, 20, 65, 100, 100, 120];
        assert_eq!(find_repeat(&input), Some(100));
    }
}
