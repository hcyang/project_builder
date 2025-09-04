/*******************************************************
    poc_algorithm_builder_rust_quick_sort_generated_program_payload.rs
*******************************************************/

/// Quick Sort implementation in Rust using Lomuto partition scheme.
/// This function sorts the input slice in place.
///
/// # Example
/// ```
/// let mut arr = vec![3, 6, 8, 10, 1, 2, 1];
/// quick_sort(&mut arr);
/// assert_eq!(arr, vec![1, 1, 2, 3, 6, 8, 10]);
/// ```

fn quick_sort(arr: &mut [i32]) {
    // Helper function for recursion
    fn sort(arr: &mut [i32], low: isize, high: isize) {
        if low < high {
            let pi = partition(arr, low, high);
            sort(arr, low, pi - 1);
            sort(arr, pi + 1, high);
        }
    }

    // Lomuto partitioning
    fn partition(arr: &mut [i32], low: isize, high: isize) -> isize {
        let pivot = arr[high as usize];
        let mut i = low - 1;
        for j in low..high {
            if arr[j as usize] <= pivot {
                i += 1;
                arr.swap(i as usize, j as usize);
            }
        }
        arr.swap((i + 1) as usize, high as usize);
        i + 1
    }

    let len = arr.len();
    if len > 1 {
        sort(arr, 0, (len - 1) as isize);
    }
}

// Example usage
fn main() {
    let mut numbers = vec![9, 4, 7, 3, 1, 5, 2, 8, 6];
    println!("Original: {:?}", numbers);

    quick_sort(&mut numbers);

    println!("Sorted:   {:?}", numbers);
}