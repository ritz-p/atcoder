use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, usize); m],
        q: usize,
    };

    let mut matrix = vec![vec![std::usize::MAX; n]; n];
    for (u, v, w) in &edges {
        matrix[*u][*v] = matrix[*u][*v].min(*w);
        matrix[*v][*u] = matrix[*v][*u].min(*w);
    }

    let mut d = vec![vec![std::usize::MAX; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for u in 0..n {
        for v in 0..n {
            if matrix[u][v] != std::usize::MAX {
                d[u][v] = matrix[u][v];
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][k] != std::usize::MAX && d[k][j] != std::usize::MAX {
                    d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
                }
            }
        }
    }

    for _ in 0..q {
        input! {
            k: usize,
            bs: [Usize1; k],
        }

        let mut dp = vec![vec![vec![std::usize::MAX; 2]; k]; 1 << k];
        for i in 0..k {
            let (u, v, w) = edges[bs[i]];
            dp[1 << i][i][0] = d[0][v] + w;
            dp[1 << i][i][1] = d[0][u] + w;
        }
        for bit in 0..1 << k {
            for i in 0..k {
                if (bit >> i) & 1 == 0 {
                    continue;
                }
                let (u0, v0, _) = edges[bs[i]];
                for j in 0..k {
                    if (bit >> j) & 1 == 1 {
                        continue;
                    }
                    if dp[bit][i][0] != std::usize::MAX {
                        let (u, v, w) = edges[bs[j]];
                        dp[bit | 1 << j][j][0] =
                            dp[bit | 1 << j][j][0].min(dp[bit][i][0] + d[u0][v] + w);
                        dp[bit | 1 << j][j][1] =
                            dp[bit | 1 << j][j][1].min(dp[bit][i][0] + d[u0][u] + w);
                    }
                    if dp[bit][i][1] != std::usize::MAX {
                        let (u, v, w) = edges[bs[j]];
                        dp[bit | 1 << j][j][0] =
                            dp[bit | 1 << j][j][0].min(dp[bit][i][1] + d[v0][v] + w);
                        dp[bit | 1 << j][j][1] =
                            dp[bit | 1 << j][j][1].min(dp[bit][i][1] + d[v0][u] + w);
                    }
                }
            }
        }
        let mut res = std::usize::MAX;
        for i in 0..k {
            res = res.min(dp[(1 << k) - 1][i][0] + d[n - 1][edges[bs[i]].0]);
            res = res.min(dp[(1 << k) - 1][i][1] + d[n - 1][edges[bs[i]].1]);
        }

        println!("{}", res);
    }
}
