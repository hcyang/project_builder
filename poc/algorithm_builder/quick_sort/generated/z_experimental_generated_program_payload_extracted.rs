// poc_quick_sort.rs
/// An efficient implementation of the Quick Sort algorithm in Rust.
/// This sorts a vector of i32s in-place using Lomuto partition scheme.

fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        // Base case: already sorted
        return;
    }
    // Partition the array
    let pivot_index = partition(arr);
    // Recursively sort the left and right partitions
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

/// Partition the slice using the last element as the pivot.
/// Returns the index of the pivot after partitioning.
fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1;
    let pivot = arr[pivot_index];
    // i: index of smaller element
    let mut i = 0;
    for j in 0..pivot_index {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    // Place the pivot after all smaller elements
    arr.swap(i, pivot_index);
    i
}

fn main() {
    // Example usage
    let mut numbers = vec![34, 7, 23, 32, 5, 62, 32, 7, 2, 1];
    println!("Before sorting: {:?}", numbers);
    quick_sort(&mut numbers);
    println!("After sorting:  {:?}", numbers);
}