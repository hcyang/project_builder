/// poc_algorithm_builder_rust_quick_sort_generated_program_payload.rs

/// Quick Sort implementation in Rust.
/// This function sorts the given slice in-place using the quick sort algorithm.
///
/// # Example
/// ```
/// let mut arr = [5, 3, 8, 4, 2];
/// quick_sort(&mut arr);
/// assert_eq!(arr, [2, 3, 4, 5, 8]);
/// ```

/// Public function to quick sort a mutable slice.
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    quick_sort_recursive(arr, 0, arr.len() - 1);
}

/// Internal recursive function for quick sort.
/// Sorts the elements between left and right indices.
fn quick_sort_recursive<T: Ord>(arr: &mut [T], left: usize, right: usize) {
    if left < right {
        let pivot_index = partition(arr, left, right);
        // Recursively sort elements before and after partition
        if pivot_index > 0 {
            quick_sort_recursive(arr, left, pivot_index - 1);
        }
        quick_sort_recursive(arr, pivot_index + 1, right);
    }
}

/// Partition the slice and return the index of the pivot element after partitioning.
/// Uses Lomuto partition scheme with the rightmost element as pivot.
fn partition<T: Ord>(arr: &mut [T], left: usize, right: usize) -> usize {
    let pivot = right;
    let mut i = left;
    for j in left..right {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot);
    i
}

fn main() {
    let mut arr = [5, 3, 8, 4, 2, 7, 1, 10];
    println!("Before sort: {:?}", arr);
    quick_sort(&mut arr);
    println!("After sort: {:?}", arr);
}
