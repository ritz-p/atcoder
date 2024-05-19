use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input!{
        n: usize,
        mut td: [(u64,u64);n]
    };
    td.sort_by_key(|&(_, d)| Reverse(d));

    let mut printed = 0;
    let mut last:u64 = 0;
    let mut heap: BinaryHeap<Reverse<u64>> = BinaryHeap::new();

    for &(enter, exit) in &td {
        if enter >= last {
            printed += 1;
            last = exit + 1;
            heap.push(Reverse(last));
        } else {
            if let Some(&Reverse(next)) = heap.peek() {
                if enter >= next {
                    heap.pop();
                    printed += 1;
                    last = next + 1;
                    heap.push(Reverse(last));
                }
            }
        }
    }
    println!("{}", printed);
}