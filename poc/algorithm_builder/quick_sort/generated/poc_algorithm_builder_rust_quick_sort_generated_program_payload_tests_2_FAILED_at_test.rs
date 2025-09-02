// poc_algorithm_builder_rust_quick_sort_generated_program_payload_tests.rs

// Unit tests for poc_algorithm_builder_rust_quick_sort_generated_program_payload.rs
//
// These tests cover all functions in the program:
// - partition
// - quick_sort
//
// Author: Test Suite Generator
// Date: 2024-06-13

// Import the functions to test.
// Assume the original functions are in a module named `quick_sort_mod`.
// If you're keeping all code in one file, you can comment out the `mod` line and
// remove the module prefix.

#[cfg(test)]
mod tests {
    // Bring functions into scope for testing
    use super::{partition, quick_sort};

    #[test]
    fn test_partition_basic() {
        // Test partition on a typical case.
        let mut arr = [9, 3, 7, 6, 2, 8];
        // partition should place all elements less than pivot (arr[5]=8) to its left
        // and return the index where pivot ends up.
        let pivot_pos = partition(&mut arr);
        // 8 should be at the pivot position
        assert_eq!(arr[pivot_pos], 8);
        // All elements to the left of 8 should be < 8, to the right >=8
        for i in 0..pivot_pos {
            assert!(arr[i] < 8);
        }
        for i in pivot_pos+1..arr.len() {
            assert!(arr[i] >= 8);
        }
    }

    #[test]
    fn test_partition_all_smaller() {
        // All elements smaller than pivot
        let mut arr = [1, 2, 3, 4, 5, 6];
        let pivot_pos = partition(&mut arr);
        assert_eq!(arr[pivot_pos], 6);
        // All left should be < 6
        for i in 0..pivot_pos {
            assert!(arr[i] < 6);
        }
        // No right elements
        assert_eq!(pivot_pos, arr.len()-1);
    }

    #[test]
    fn test_partition_all_greater() {
        // All elements greater than pivot
        let mut arr = [10, 11, 12, 13, 1];
        let pivot_pos = partition(&mut arr);
        assert_eq!(arr[pivot_pos], 1);
        // All left should be < 1 (none)
        assert_eq!(pivot_pos, 0);
        // All right should be >= 1
        for i in pivot_pos+1..arr.len() {
            assert!(arr[i] >= 1);
        }
    }

    #[test]
    fn test_partition_duplicates() {
        // Duplicates
        let mut arr = [4, 2, 4, 4, 2, 4];
        let pivot_pos = partition(&mut arr);
        assert_eq!(arr[pivot_pos], 4);
        for i in 0..pivot_pos {
            assert!(arr[i] < 4);
        }
        for i in pivot_pos+1..arr.len() {
            assert!(arr[i] >= 4);
        }
    }

    #[test]
    fn test_partition_single_element() {
        let mut arr = [5];
        // partition should not panic and return index 0
        let pivot_pos = partition(&mut arr);
        assert_eq!(pivot_pos, 0);
        assert_eq!(arr, [5]);
    }

    #[test]
    #[should_panic]
    fn test_partition_empty_slice_panics() {
        // partition will panic if slice is empty (len-1 underflow)
        let mut arr: [i32; 0] = [];
        let _ = partition(&mut arr);
    }

    #[test]
    fn test_quick_sort_basic() {
        let mut arr = [4, 1, 8, 2, 9, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 8, 9]);
    }

    #[test]
    fn test_quick_sort_already_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_reverse_sorted() {
        let mut arr = [9, 8, 7, 6, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_quick_sort_with_negatives() {
        let mut arr = [0, -10, 5, -3, 2];
        quick_sort(&mut arr);
        assert_eq!(arr, [-10, -3, 0, 2, 5]);
    }

    #[test]
    fn test_quick_sort_all_equal() {
        let mut arr = [7, 7, 7, 7, 7];
        quick_sort(&mut arr);
        assert_eq!(arr, [7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_quick_sort_empty() {
        let mut arr: [i32; 0] = [];
        quick_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_quick_sort_single_element() {
        let mut arr = [42];
        quick_sort(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_quick_sort_large_random() {
        use rand::seq::SliceRandom;
        let mut arr: Vec<i32> = (0..1000).collect();
        let mut rng = rand::thread_rng();
        arr.shuffle(&mut rng);
        quick_sort(&mut arr);
        assert_eq!(arr, (0..1000).collect::<Vec<_>>());
    }
}