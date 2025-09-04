// poc_algorithm_builder_rust_quick_sort_generated_program_payload_tests.rs

// Required imports for testing
#[cfg(test)]
mod tests {
    // Import functions from the parent module if necessary.
    // If the functions are in the same file (as in this case), just declare them again for test visibility.
    // If using `mod` structure, use `super::*;`.

    // Re-declare the functions to make them accessible for unit tests.
    // In real usage, these would be imported from the main module.

    /// Quick Sort implementation in Rust
    fn quick_sort(arr: &mut [i32]) {
        if arr.len() > 1 {
            let pivot_index = partition(arr);
            quick_sort(&mut arr[..pivot_index]);
            quick_sort(&mut arr[pivot_index + 1..]);
        }
    }

    /// Partition function using Lomuto partition scheme.
    fn partition(arr: &mut [i32]) -> usize {
        let len = arr.len();
        let pivot = arr[len - 1];
        let mut i = 0;
        for j in 0..len - 1 {
            if arr[j] <= pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, len - 1);
        i
    }

    // Unit tests for quick_sort and partition

    #[test]
    fn test_quick_sort_empty_array() {
        let mut arr: Vec<i32> = vec![];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_quick_sort_single_element() {
        let mut arr = vec![42];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_quick_sort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_duplicates() {
        let mut arr = vec![3, 6, 8, 10, 1, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 6, 8, 10]);
    }

    #[test]
    fn test_quick_sort_all_equal() {
        let mut arr = vec![7, 7, 7, 7];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![7, 7, 7, 7]);
    }

    #[test]
    fn test_quick_sort_large_array() {
        let mut arr = vec![29, 10, 14, 37, 13, 3, 8, 1, 2, 11];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 8, 10, 11, 13, 14, 29, 37]);
    }

    #[test]
    fn test_partition_basic() {
        let mut arr = vec![3, 2, 1];
        // After partition, pivot (1) should be at index 0, and [3,2] should be right of it.
        let idx = partition(&mut arr);
        // 1 should be at the start
        assert_eq!(arr[0], 1);
        // Partition index should be 0
        assert_eq!(idx, 0);
        // Remaining elements should be [3, 2] in some order
        assert!(arr[1..].contains(&2) && arr[1..].contains(&3));
    }

    #[test]
    fn test_partition_all_less_than_pivot() {
        let mut arr = vec![1, 2, 3, 4];
        // Pivot is 4 (last element), all others are less
        let idx = partition(&mut arr);
        // Pivot should be at the end
        assert_eq!(arr[3], 4);
        // Partition index should be 3
        assert_eq!(idx, 3);
        // Order of the first three elements should be preserved
        assert_eq!(arr[..3], [1, 2, 3]);
    }

    #[test]
    fn test_partition_all_greater_than_pivot() {
        let mut arr = vec![5, 6, 7, 1];
        // Pivot is 1
        let idx = partition(&mut arr);
        // Pivot should be at the start
        assert_eq!(arr[0], 1);
        // Partition index should be 0
        assert_eq!(idx, 0);
        // The rest should be [6, 7, 5] in some order
        assert!(arr[1..].contains(&5));
        assert!(arr[1..].contains(&6));
        assert!(arr[1..].contains(&7));
    }

    #[test]
    fn test_partition_duplicates() {
        let mut arr = vec![4, 4, 4, 4];
        let idx = partition(&mut arr);
        // All elements are equal, pivot can be anywhere (should be at end, index 3)
        assert_eq!(idx, 3);
        // Array should still be all 4
        assert_eq!(arr, vec![4, 4, 4, 4]);
    }

    #[test]
    fn test_partition_two_elements() {
        let mut arr = vec![2, 1];
        let idx = partition(&mut arr);
        // After partition, 1 should be at index 0 (pivot), 2 at index 1
        assert_eq!(arr, vec![1, 2]);
        assert_eq!(idx, 0);
    }
}