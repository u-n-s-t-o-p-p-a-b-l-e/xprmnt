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
}
