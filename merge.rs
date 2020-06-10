// bubble sort
pub fn merge_sort(array: &mut [i32]) {
    let len = array.len();
    let middle = len / 2;
    if len < 2 {
        return;
    }

    let mut sorted = array.to_vec();
    merge_sort(&mut array[..middle]);
    merge_sort(&mut array[middle..]);
    merge(&array[..middle], &array[middle..], &mut sorted);

    array.copy_from_slice(&sorted);
}

fn merge(left_arr: &[i32], r_arr: &[i32], sorted: &mut [i32]) {
    let (mut left, mut right, mut i) = (0, 0, 0);

    while left < left_arr.len() && right < r_arr.len() {
        if left_arr[left] <= r_arr[right] {
            sorted[i] = left_arr[left];
            left += 1;
        } else {
            sorted[i] = r_arr[right];
            right += 1;
        }
        i += 1;
    }

    if left < left_arr.len() {
        sorted[i..].copy_from_slice(&left_arr[left..]);
    }

    if right < r_arr.len() {
        sorted[i..].copy_from_slice(&r_arr[right..]);
    }
}
