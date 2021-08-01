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
