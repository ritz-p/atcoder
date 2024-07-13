use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize;n],
        uvb: [(usize,usize,usize);m]
    };
    let mut graph = vec![vec![]; n];
    for (u, v, b) in &uvb {
        graph[u - 1].push((*v - 1, *b));
        graph[v - 1].push((*u - 1, *b));
    }

    let res = dijkstra(&graph, &a, 0);

    println!("{}", res.iter().skip(1).join(" "));
}

fn dijkstra(
    graph: &Vec<Vec<(usize, usize)>>,
    node_weights: &Vec<usize>,
    start: usize,
) -> Vec<usize> {
    let mut dist: Vec<usize> = vec![usize::MAX; graph.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = node_weights[start];
    heap.push((Reverse(node_weights[start]), start));
    while let Some((Reverse(cost), position)) = heap.pop() {
        if cost > dist[position] {
            continue;
        }
        for &(neighbor, edge_cost) in &graph[position] {
            let next_cost = cost + edge_cost + node_weights[neighbor];
            if next_cost < dist[neighbor] {
                heap.push((Reverse(next_cost), neighbor));
                dist[neighbor] = next_cost;
            }
        }
    }

    dist
}
