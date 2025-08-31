// poc_algorithm_builder_rust_quick_sort_generated_program_payload.rs
//
// An efficient implementation of the Quick Sort algorithm in Rust.
//
// Author: Your Name
// Date: 2024-06
//
// Quick Sort is a divide-and-conquer sorting algorithm with
// average time complexity O(n log n). This implementation sorts
// a mutable slice in-place.

/// Performs quick sort on a mutable slice of elements.
/// The elements must implement the `PartialOrd` and `Copy` traits.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements to be sorted.
///
/// # Example
///
/// ```
/// let mut numbers = vec![3, 6, 1, 5, 2, 4];
/// quick_sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6]);
/// ```
pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let len = arr.len();
    // Partition the array and get the pivot index
    let pivot_index = partition(arr);
    // Recursively sort elements before and after partition
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..len]);
}

/// Partitions the slice around a pivot chosen as the last element.
/// Elements less than the pivot are moved to the left,
/// and elements greater are moved to the right.
///
/// # Returns
///
/// * The index of the pivot after partitioning.
fn partition<T: PartialOrd + Copy>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;
    for j in 0..len-1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

fn main() {
    let mut numbers = vec![34, 7, 23, 32, 5, 62];
    println!("Before sorting: {:?}", numbers);

    quick_sort(&mut numbers);

    println!("After sorting:  {:?}", numbers);
}
