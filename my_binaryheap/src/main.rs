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

    let mut jobs = BinaryHeap::new();

    jobs.push((100, "Reply to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((10, "Watch some Youtube"));
    jobs.push((70, "Tell your team members thanks for always working_hard"));
    jobs.push((30, "Plan whho to hire next for the team"));

    for (_, job) in jobs {
        println!("You need to do: {job}");
    }
}
