//// ## poc_algorithm_builder_rust_quick_sort_generated_program_payload.rs
//// ```
// Quick Sort implementation in Rust
// Efficient, generic, in-place, and well-documented

/// Performs in-place Quick Sort on a mutable slice.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements implementing the `Ord` trait.
///
/// # Example
///
/// ```
/// let mut data = [5, 2, 9, 1, 5, 6];
/// quick_sort(&mut data);
/// assert_eq!(data, [1, 2, 5, 5, 6, 9]);
/// ```
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        // Already sorted
        return;
    }
    // Partition the array and sort the partitions recursively
    let pivot_index = partition(arr);
    // Recursively sort elements before and after partition
    quick_sort(&mut arr[..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

/// Partitions the slice around a pivot chosen from the end.
/// All elements less than the pivot are moved before it,
/// all greater after.
///
/// Returns the index of the pivot after partitioning.
///
/// # Arguments
///
/// * `arr` - The mutable slice to partition.
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1;
    // Choose the pivot (last element)
    let mut i = 0;
    for j in 0..pivot_index {
        if arr[j] < arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

fn main() {
    let mut numbers = [10, 7, 8, 9, 1, 5];
    println!("Before sorting: {:?}", numbers);
    quick_sort(&mut numbers);
    println!("After sorting:  {:?}", numbers);

    let mut words = ["pear", "apple", "orange", "banana"];
    println!("Before sorting: {:?}", words);
    quick_sort(&mut words);
    println!("After sorting:  {:?}", words);
}
//// ```
