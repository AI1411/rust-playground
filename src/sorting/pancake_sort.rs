pub fn pancake_sort<T>(arr: &mut [T]) -> Vec<T>
    where T: PartialEq + Ord + PartialOrd + Clone,
{
    let len = arr.len();
    if len < 2 {
        return arr.to_vec();
    }
    for i in (0..len).rev() {
        let max_index = arr
            .iter()
            .take(i + 1)
            .enumerate()
            .max_by_key(|&(_, elem)| elem)
            .map(|(idx, _)| idx)
            .unwrap();
        if max_index != i {
            arr[0..max_index + 1].reverse();
            arr[0..i + 1].reverse();
        }
    }
    arr.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let res = pancake_sort(&mut vec![6, 5, -8, 3, 2, 3]);
        assert_eq!(res, vec![-8, 2, 3, 3, 5, 6]);
    }

    #[test]
    fn already_sorted() {
        let res = pancake_sort(&mut vec![1, 2, 3, 4, 5]);
        assert_eq!(res, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn odd_number_of_element() {
        let res = pancake_sort(&mut vec!["d", "a", "c", "e", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn one_element() {
        let res = pancake_sort(&mut vec![1]);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn empty() {
        let res = pancake_sort(&mut Vec::<u8>::new());
        assert_eq!(res, vec![]);
    }
}