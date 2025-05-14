use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize);n-1]
    };
    let mut graph = vec![vec![]; n];

    for (a, b) in &ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    for g in &graph {
        if g.len() == n - 1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
