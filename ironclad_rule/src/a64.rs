use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};
fn main(){
    input!{
        n: usize,
        m: usize,
        abc: [(usize,usize,usize);m]
    };
    let mut graph = vec![vec![];n];
    for (a,b,c) in abc{
        graph[a-1].push((b-1,c));
        graph[b-1].push((a-1,c));
    }
    let mut distance = vec![1000000001;n];
    let mut heap = BinaryHeap::new();
    distance[0] = 0;
    heap.push((Reverse(0),0));

    while let Some((Reverse(d),v)) = heap.pop(){
        if d > distance[v]{
            continue;
        }
        for (x,y) in &graph[v]{
            if distance[*x] > distance[v] + y{
                distance[*x] = distance[v] + y;
                heap.push((Reverse(distance[*x]),*x));
            } 
        }
    }
    for d in distance{
        if d == 1000000001{
            println!("-1");
        }else{
            println!("{}",d);
        }
    }
}