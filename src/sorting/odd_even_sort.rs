pub fn odd_even_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len == 0 {
        return;
    }

    let mut sorted = false;
    while !sorted {
        sorted = true;

        for i in (1..len - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }

        for i in (0..len - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![3, 2, 1, 4, 5, 6, 7, 8, 9, 10];
        odd_even_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn empty() {
        let mut arr =  Vec::<i32>::new();
        odd_even_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        odd_even_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        odd_even_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn reverse_sorted() {
        let mut arr = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        odd_even_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}