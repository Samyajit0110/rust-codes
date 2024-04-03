fn find_median(arr: &[i32]) -> Option<f64> {
    let len = arr.len();

    if len == 0 {
        return None;
    }

    if len % 2 == 0 {
        // Array has an even number of elements
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        Some((arr[mid_left] + arr[mid_right]) as f64 / 2.0)
    } else {
        // Array has an odd number of elements
        let mid = len / 2;
        Some(arr[mid] as f64)
    }
}

fn main() {
    let sorted_array1 = vec![1, 2, 3, 4, 5];
    let sorted_array2 = vec![1, 2, 3, 4, 5, 6];

    match find_median(&sorted_array1) {
        Some(median) => println!("Median of {:?}: {}", sorted_array1, median),
        None => println!("Array is empty"),
    }

    match find_median(&sorted_array2) {
        Some(median) => println!("Median of {:?}: {}", sorted_array2, median),
        None => println!("Array is empty"),
    }
}
