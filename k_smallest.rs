use rand::seq::SliceRandom;
use rand::thread_rng;

fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }

    let mut rng = thread_rng();
    let mut shuffled_arr = arr.to_vec();
    shuffled_arr.shuffle(&mut rng);

    let mut sorted_arr = shuffled_arr.clone();
    sorted_arr.sort();

    Some(sorted_arr[k - 1])
}

fn main() {
    let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let k = 5;

    match kth_smallest_element(&arr, k) {
        Some(kth_smallest) => println!("The {}th smallest element is: {}", k, kth_smallest),
        None => println!("The array is not large enough for {}th smallest element.", k),
    }
}
