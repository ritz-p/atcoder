use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [(usize,char,usize,char);m]
    };
    let mut graph = vec![vec![]; n];
    let mut deg = vec![0; n];

    for (a, _b, c, _d) in abcd {
        graph[a - 1].push(c - 1);
        graph[c - 1].push(a - 1);
        deg[a - 1] += 1;
        deg[c - 1] += 1;
    }
    let mut x = 0;
    let mut y = 0;

    let mut visited = vec![false; n];

    for i in 0..n {
        if !visited[i] {
            let mut queue = VecDeque::new();
            queue.push_back(i);
            visited[i] = true;
            let mut f = true;
            while let Some(v) = queue.pop_front() {
                if deg[v] != 2 {
                    f = false;
                }
                for e in &graph[v] {
                    if !visited[*e] {
                        queue.push_back(*e);
                        visited[*e] = true;
                    }
                }
            }
            if f {
                x += 1;
            } else {
                y += 1;
            }
        }
    }

    println!("{} {}", x, y);
}
