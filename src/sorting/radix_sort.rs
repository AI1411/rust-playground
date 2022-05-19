pub fn radix_sort(arr: &mut [u64]) {
    let max: usize = match arr.iter().max() {
        Some(&x) => x as usize,
        None => return,
    };
    let radix = arr.len().next_power_of_two();
    let mut place = 1;
    while place <= max {
        let digit_of = |x| x as usize / place % radix;
        let mut counter = vec![0; radix];
        for &x in arr.iter() {
            counter[digit_of(x)] += 1;
        }
        for i in 1..radix {
            counter[i] += counter[i - 1];
        }
        for &x in arr.to_owned().iter().rev() {
            counter[digit_of(x)] -= 1;
            arr[counter[digit_of(x)]] = x;
        }
        place *= radix;
    }
}

#[cfg(test)]
mod tests {
    use super::radix_sort;
    use super::super::is_sorted;

    #[test]
    fn empty() {
        let mut a: [u64; 0] = [];
        radix_sort(&mut a);
        assert!(is_sorted(&a));
    }

    #[test]
    fn descending() {
        let mut a = vec![201, 127, 64, 37, 24, 4, 1];
        radix_sort(&mut a);
        assert!(is_sorted(&a));
    }

    #[test]
    fn ascending() {
        let mut a = vec![1, 4, 24, 37, 64, 127, 201];
        radix_sort(&mut a);
        assert!(is_sorted(&a));
    }
}