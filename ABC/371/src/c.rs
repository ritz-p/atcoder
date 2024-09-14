use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mg: usize,
        guv: [(usize,usize);mg],
        mh: usize,
        huv: [(usize,usize);mh],
    };
    let mut costs = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input! {
            a: [usize; n - i - 1]
        };
        for (j, &c) in a.iter().enumerate() {
            let u = i;
            let v = i + j + 1;
            costs[u][v] = c;
            costs[v][u] = c;
        }
    }

    let mut g = vec![vec![false; n]; n];
    let mut h = vec![vec![false; n]; n];
    for (u, v) in guv {
        g[u - 1][v - 1] = true;
        g[v - 1][u - 1] = true;
    }
    for (u, v) in huv {
        h[u - 1][v - 1] = true;
        h[v - 1][u - 1] = true;
    }

    let perm: Vec<usize> = (0..n).collect();
    let mut cost = usize::MAX;
    for p in perm.iter().permutations(n) {
        let mut current = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if h[i][j] != g[*p[i]][*p[j]] {
                    current += costs[i][j];
                }
            }
        }
        cost = cost.min(current);
    }

    println!("{}", cost);
}
