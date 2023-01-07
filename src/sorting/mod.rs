mod bogo_sort;
mod bucket_sort;
mod buuble_sort;

use std::cmp;

pub use self::bogo_sort::bogo_sort;
pub use self::bucket_sort::bucket_sort;
pub use self::buuble_sort::bubble_sort;

pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: cmp::PartialOrd,
{
    if arr.is_empty() {
        return true;
    }

    let mut prev = &arr[0];

    for item in arr.iter().skip(1) {
        if prev > item {
            return false;
        }
        prev = item;
    }

    true
}
