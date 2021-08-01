use std::collections::HashSet;

pub fn find_repeat(list: &[i32]) -> Option<i32> {
    let mut history: HashSet<i32> = HashSet::new();
    let mut iter = list.iter();

    loop {
        match iter.next() {
            Some(item) => {
                match history.get(item) {
                    Some(anything) => return Some(*anything),
                    None => history.insert(*item),
                };
            }
            None => return None,
        }
    }
}
