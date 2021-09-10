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
