fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result = None;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1; 
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    let sorted_array = vec![1, 2, 2, 3, 4, 5, 5, 6, 7, 8, 9, 9, 10];
    let target = 5;

    match find_first_occurrence(&sorted_array, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}

