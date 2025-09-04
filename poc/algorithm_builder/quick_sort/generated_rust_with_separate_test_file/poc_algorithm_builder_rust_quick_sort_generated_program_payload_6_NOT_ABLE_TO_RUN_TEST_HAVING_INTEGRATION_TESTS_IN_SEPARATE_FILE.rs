// poc_algorithm_builder_rust_quick_sort_generated_program_payload.rs

/// Quick Sort implementation in Rust
/// Sorts a mutable slice of integers in ascending order.
///
/// # Example
/// ```
/// let mut arr = vec![3, 6, 8, 10, 1, 2, 1];
/// quick_sort(&mut arr);
/// assert_eq!(arr, vec![1, 1, 2, 3, 6, 8, 10]);
/// ```
fn quick_sort(arr: &mut [i32]) {
    // Call the recursive quicksort function
    if arr.len() > 1 {
        // Partition and recursively sort the subarrays
        let pivot_index = partition(arr);
        // Recursively sort left part
        quick_sort(&mut arr[..pivot_index]);
        // Recursively sort right part
        quick_sort(&mut arr[pivot_index + 1..]);
    }
}

/// Partition function using Lomuto partition scheme.
/// Picks the last element as pivot, places it at the correct position,
/// and all smaller elements to its left and greater to its right.
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

fn main() {
    let mut arr = vec![29, 10, 14, 37, 13, 3, 8, 1, 2, 11];
    println!("Original array: {:?}", arr);

    quick_sort(&mut arr);

    println!("Sorted array:   {:?}", arr);
}
