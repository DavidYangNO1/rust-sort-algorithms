// bubble sort
pub fn bubble_sort(arrary: &mut Vec<i32>) {
    let len = arrary.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arrary[j + 1] < arrary[j] {
                arrary.swap(j, j + 1);
            }
        }
    }
}
