use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n1 + n2];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    let max = 300001;
    let mut distance = vec![max;n1+n2];
    distance[0] = 0;
    let mut n1_count = 0;
    let mut current = vec![];
    let mut visited = HashSet::new();
    current.push(0);

    while visited.len() < n1{
        let mut next = vec![];
        for e in current{
            if visited.contains(&e){
                continue;
            }

            for ne in &graph[e]{
                if distance[*ne] > n1_count+1{
                    distance[*ne] = n1_count+1;
                }
                next.push(*ne);
            }
            visited.insert(e);
        }
        n1_count += 1;
        current = next;
    }

    distance[n1+n2-1] = 0;
    let mut n2_count = 0;
    current=[n1+n2-1].to_vec();
    while visited.len() < n1+n2{
        let mut next = vec![];
        for e in current{
            if visited.contains(&e){
                continue;
            }
            for ne in &graph[e]{
                if distance[*ne] > n2_count{
                    distance[*ne] = n2_count;
                }
                next.push(*ne);
            }
            visited.insert(e);
        }
        n2_count += 1;
        current = next;
    }

    println!("{}",n1_count+n2_count-1);
}
