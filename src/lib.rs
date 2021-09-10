#![allow(dead_code)]
mod algorithms;
mod exercises;
#[cfg(test)]
mod tests_sort {
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

#[cfg(test)]
mod tests_exercises {
    use crate::exercises::first_repeating_token::find_repeat;

    #[test]
    fn found_int_result() {
        let input: [i32; 6] = [12, 20, 65, 100, 100, 120];
        assert_eq!(find_repeat(&input), Some(100));
    }
    #[test]
    fn no_result() {
        let input: [i32; 5] = [12, 20, 65, 100, 120];
        assert_eq!(find_repeat(&input), None);
    }

    #[test]
    fn interweaved_duplicates() {
        let input: [i32; 5] = [5, 12, 13, 5, 12];
        assert_eq!(find_repeat(&input), Some(5));
    }
    #[test]
    fn nested_duplicates() {
        let input: [i32; 5] = [5, 12, 13, 12, 5];
        assert_eq!(find_repeat(&input), Some(12));
    }
    #[test]
    fn try_with_strings() {
        let input: [&str; 5] = ["five", "twelve", "thirteen", "twelve", "five"];
        assert_eq!(find_repeat(&input), Some("twelve"));
    }
    #[test]
    fn try_with_chars() {
        let input: [char; 5] = ['f', 't', 'e', 't', 'f'];
        assert_eq!(find_repeat(&input), Some('t'));
    }
}
