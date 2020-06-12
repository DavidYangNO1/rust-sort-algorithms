// quick sort
pub fn quick_sort(array: &mut [usize]) {
    sort(array, 0, array.len() - 1);
}

fn sort(array: &mut [usize], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let j = partition(array, lo, hi);

    if j > 0 {
        sort(array, lo, j - 1);
    }
    sort(array, j + 1, hi);
}

fn partition(array: &mut [usize], lo: usize, hi: usize) -> usize {
    let (mut i, mut j) = (lo, hi + 1);
    let v = array[lo];
    loop {
        loop {
            i += 1;
            if !(array[i] < v) || i == hi {
                break;
            }
        }
        loop {
            j -= 1;
            if !(v < array[j]) || j == lo {
                break;
            }
        }

        if i >= j {
            break;
        }
        array.swap(i, j);
    }
    array.swap(lo, j);
    return j;
}
