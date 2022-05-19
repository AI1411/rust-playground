pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec!["d", "a", "c", "b"];
        selection_sort(&mut arr);
        assert_eq!(arr, ["a", "b", "c", "d"]);
    }

    #[test]
    fn empty() {
        let mut arr = Vec::<u8>::new();
        selection_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        selection_sort(&mut res);
        assert_eq!(res, [1]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4, 5];
        selection_sort(&mut res);
        assert_eq!(res, [1, 2, 3, 4, 5]);
    }
}