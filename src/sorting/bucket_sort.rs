pub fn bucket_sort(arr: &[usize]) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn empty() {
        let arr: [usize; 0] = [];
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res));
    }

    #[test]
    fn one_element() {
        let arr = [4];
        let res = bucket_sort(&arr);

        assert!(is_sorted(&res));
    }

    #[test]
    fn already_sorted() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8];
        let res = bucket_sort(&arr);

        assert!(is_sorted(&res));
    }

    #[test]
    fn basic() {
        let res = bucket_sort(&[35, 53, 7, 3, 4]);
        assert!(is_sorted(&res));
    }

    #[test]
    fn odd_number_of_elements() {
        let res = bucket_sort(&[1, 21, 5, 11, 58]);
        assert!(is_sorted(&res));
    }

    #[test]
    fn repeated_elements() {
        let res = bucket_sort(&[542, 542, 542, 542]);
        assert!(is_sorted(&res));
    }
}
