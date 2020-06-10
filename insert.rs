// insert sort
pub fn insert_sort(arrary: &mut Vec<i32>) {
    let len = arrary.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 {
            if arrary[j] < arrary[j - 1] {
                arrary.swap(j, j - 1);
            }
            j -= 1;
        }
    }
}
