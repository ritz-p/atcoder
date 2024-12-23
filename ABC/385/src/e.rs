use proconio::input;

fn main() {
    input! {
        n: usize,
        uv: [(usize,usize);n-1]
    };
    let mut graph = vec![vec![]; n];
    let mut l = vec![0; n];
    for (u, v) in uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
        l[u - 1] += 1;
        l[v - 1] += 1;
    }

    for i in 0..n {
        graph[i].sort_by(|&a, &b| l[b].cmp(&l[a]));
    }

    let mut res = n;

    for i in 0..n {
        graph[i].sort_by_key(|&i| l[i]);
        let mut x = 0;
        let mut y = 0;
        while let Some(v) = graph[i].pop() {
            x += 1;
            y = l[v] - 1;
            res = res.min(n - x - x * y - 1)
        }
    }

    println!("{}", res);
}
