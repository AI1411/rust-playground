use std::cmp::Ordering;

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search(&"a", &vec![]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = binary_search(&"a", &vec!["a"]);
        assert_eq!(index, Some(0))
    }
}