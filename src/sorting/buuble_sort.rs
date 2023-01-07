pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    let mut sorted = false;
    let mut n = arr.len();

    while !sorted {
        sorted = true;

        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false
            }
        }

        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn descending() {
        let mut vec1 = vec![6, 5, 4, 3, 2, 1];

        bubble_sort(&mut vec1);
        assert!(is_sorted(&vec1));
    }

    #[test]
    fn ascending() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn empty() {
        let mut vec: Vec<usize> = vec![];
        bubble_sort(&mut vec);

        assert!(is_sorted(&vec));
    }
}
