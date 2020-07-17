mod bubble;
mod insert;
mod merge;
mod quick;
mod heap;

use bubble::bubble_sort;
use insert::insert_sort;
use merge::merge_sort;
use quick::quick_sort;

fn main() {
   
    println!("test heap sort algorithms");
    test_heap();

    // println!("test bubble sort algorithms");
    // test_bubble();

    // println!("test merge sort");
    // test_merge();

    // println!("test insert sort");
    // test_insert();

    // println!("test quick sort");
    // test_quick();
}

fn test_heap() {
    let mut array = vec![1, 10, 3, 28, 29, 30, 11, 100, 23];
    println!("before heap sort arrary {:?}", array);
    heap::heap_sort(&mut array);
    println!("after heap sort arrary  {:?}", array);
}

fn test_quick() {
    let mut array = vec![1, 10, 3, 28, 29, 30, 11, 100, 23];
    println!("before quick sort arrary {:?}", array);
    quick_sort(&mut array);
    println!("after quick sort arrary  {:?}", array);
}

fn test_insert() {
    let mut array = vec![1, 10, 3, 28, 29, 30, 11, 100, 23];
    println!("before insert sort arrary {:?}", array);
    insert_sort(&mut array);
    println!("after insert sort arrary  {:?}", array);
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
