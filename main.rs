mod bubble;
use bubble::bubble_sort;
mod merge;
use merge::merge_sort;

fn main() {
    println!("test bubble sort algorithms");
    test_bubble();

    println!("test merge sort");
    test_merge();
}

fn test_bubble() {
    let mut array = vec![1, 10, 3, 28, 29, 30, 11, 100, 23];
    println!("before bubble sort arrary {:?}", array);
    bubble_sort(&mut array);
    println!("after bubble sort arrary  {:?}", array);
}

fn test_merge() {
    let mut array = [1, 10, 3, 28, 29, 30, 11, 100, 23];
    println!("before merge arrary {:?}", array);
    merge_sort(&mut array);
    println!("after merge sort arrary  {:?}", array);
}
