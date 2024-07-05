use std::time::Instant;

fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);

    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition(arr: &mut [i32]) ->  usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    let mut store_index = 0;
    for i in 0..len - 1 {
        if arr[i] < arr[len -1] {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }
    arr.swap(store_index, len -1);
    store_index
}

fn main() {
    let mut arr = vec![5, 3, 8, 2, 7, 1, 10, 6, 9];
    println!("Original array: {:?}", arr);

    let start = Instant::now();
    quicksort(&mut arr);
    let duration = start.elapsed();

    println!("Sorted array: {:?}", arr);
    println!("Time taken: {:?}", duration);
}
