// poc_algorithm_builder_rust_quick_sort_generated_program_payload.rs

/// Performs in-place Quick Sort on a mutable slice.
///
/// # Arguments
/// * `arr` - The mutable slice to be sorted.
///
/// # Example
/// ```
/// let mut nums = vec![3, 1, 4, 1, 5, 9, 2];
/// quick_sort(&mut nums);
/// assert_eq!(nums, vec![1, 1, 2, 3, 4, 5, 9]);
/// ```
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    // Partition the slice and get the pivot index
    let pivot_index = partition(arr);
    // Recursively sort the two partitions
    quick_sort(&mut arr[..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

/// Partitions the slice around a pivot (last element),
/// such that all elements less than the pivot come before it,
/// and all greater come after.
///
/// Returns the final pivot index.
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1;
    let mut i = 0;
    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

fn main() {
    let mut nums = vec![10, 7, 8, 9, 1, 5];
    println!("Before sorting: {:?}", nums);

    quick_sort(&mut nums);

    println!("After sorting:  {:?}", nums);
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn test_quick_sort_empty() {
        let mut v: Vec<i32> = vec![];
        quick_sort(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_quick_sort_single() {
        let mut v = vec![42];
        quick_sort(&mut v);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn test_quick_sort_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_reverse() {
        let mut v = vec![5, 4, 3, 2, 1];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_duplicates() {
        let mut v = vec![3, 1, 2, 1, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 3, 3]);
    }
}