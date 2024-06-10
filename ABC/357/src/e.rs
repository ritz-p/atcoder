use std::collections::HashSet;

use ac_library::SccGraph;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
    }

    let mut graph = SccGraph::new(n);
    for (index, &j) in a.iter().enumerate() {
        graph.add_edge(index, j-1);
    }
    let scc = graph.scc();

    let mut counts = vec![0usize;n];
    for v in scc.iter().rev() {
        let mut set = HashSet::new();
        for x in v {
            set.insert(x);
        }

        let mut count = 0;
        for &x in v {
            let x = a[x];
            if !set.contains(&x) {
                count = counts[x];
                break;
            }
        }
        count += v.len();

        for &x in v {
            counts[x] = count;
        }
    }

    let result: usize = counts.iter().sum();
    println!("{result}");
}
