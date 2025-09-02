// poc_algorithm_builder_rust_quick_sort_generated_program_payload.rs

// Quick Sort implementation in Rust
//
// This program demonstrates the quick sort algorithm on a vector of integers.
// It is efficient and uses in-place partitioning.
//
// Author: Your Name
// Date: YYYY-MM-DD

/// Partition the slice around a pivot
///
/// Rearranges elements so that those less than the pivot are on the left,
/// and those greater are on the right.
///
/// Returns the final pivot position.
fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1; // Choosing the last element as the pivot
    let pivot = arr[pivot_index];
    let mut i = 0;

    // Move elements less than pivot to the left
    for j in 0..pivot_index {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

/// Quick sort function
///
/// Sorts the slice in-place using quick sort algorithm.
fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..len]);
}

fn main() {
    let mut numbers = vec![34, 7, 23, 32, 5, 62, 32, -4, 0, 99];

    println!("Original array: {:?}", numbers);

    quick_sort(&mut numbers);

    println!("Sorted array:   {:?}", numbers);
}