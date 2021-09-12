use std::collections::HashSet;
use std::hash::Hash;

pub fn find_repeat<T: Eq + Hash + Copy>(list: &[T]) -> Option<T> {
    let mut history: HashSet<&T> = HashSet::new();
    let mut iter = list.iter();

    loop {
        match iter.next() {
            Some(item) => {
                match history.get(item) {
                    Some(anything) => return Some(**anything),
                    None => history.insert(item),
                };
            }
            None => return None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

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
