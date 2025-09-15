use std::collections::BinaryHeap;

fn main() {
    let many_number = vec![0, 5, 10, 15, 20, 25, 30];

    let mut binary_heap = BinaryHeap::new();

    for num in many_number {
        binary_heap.push(num);
    }

    println!("First item is largest, others are out of order: {binary_heap:?}");

    while let Some(num) = binary_heap.pop() {
        println!("Popped off {num}. Remaining numbers are: {binary_heap:?}");
    }
}
