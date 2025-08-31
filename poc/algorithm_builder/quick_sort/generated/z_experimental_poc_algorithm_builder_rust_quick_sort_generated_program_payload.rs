// poc_algorithm_builder_rust_quick_sort_generated_program_payload.rs
// quick_sort.rs
/// Quick Sort implementation in Rust
///
/// This function sorts a mutable slice in-place using the quick sort algorithm.
/// It is generic over any type that implements the `Ord` trait.

/// Partition the slice and return the pivot index.
/// All elements less than the pivot are moved to the left,
/// all elements greater are moved to the right.
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2; // Choose the middle element as pivot for better performance
    arr.swap(pivot_index, len - 1); // Move pivot to the end
    let mut store_index = 0;

    for i in 0..len - 1 {
        if arr[i] < arr[len - 1] {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }
    arr.swap(store_index, len - 1); // Move pivot to its final place
    store_index
}

/// The main quick_sort function (recursive)
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return; // Already sorted
    }
    let pivot_index = partition(arr);
    // Recursively sort elements before and after partition
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..len]);
}

// Example usage and test
fn main() {
    let mut numbers = vec![10, 7, 8, 9, 1, 5];
    println!("Unsorted: {:?}", numbers);
    quick_sort(&mut numbers);
    println!("Sorted: {:?}", numbers);

    // Test with other types
    let mut strings = vec!["pear", "apple", "orange", "banana"];
    quick_sort(&mut strings);
    println!("Sorted strings: {:?}", strings);
}