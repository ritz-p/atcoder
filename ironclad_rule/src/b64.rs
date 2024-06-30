use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize,usize,usize);m]
    };

    let mut graph = vec![vec![]; n];
    let mut distance = vec![1000000001; n];
    for (a, b, c) in abc {
        graph[a - 1].push((b - 1, c));
        graph[b - 1].push((a - 1, c));
    }
    let mut heap = BinaryHeap::new();
    distance[0] = 0;
    heap.push((Reverse(0), 0));

    // 毎回最小距離を求める
    while let Some((Reverse(d), v)) = heap.pop() {
        if d > distance[v] {
            continue;
        }
        for (x, y) in &graph[v] {
            if distance[*x] > distance[v] + y {
                distance[*x] = distance[v] + y;
                heap.push((Reverse(distance[*x]), *x));
            }
        }
    }
    let mut current = n - 1;
    let mut route = vec![];
    route.push(n);
    while current != 0 {
        for (x, y) in &graph[current] {
            if distance[*x] + y == distance[current] {
                route.push(*x + 1);
                current = *x;
                break;
            }
        }
    }

    println!("{}", route.iter().rev().join(" "));
}
