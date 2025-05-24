use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize;w];h]
    };
    let l = 1usize << w;
    let mut dp = vec![vec![-1; l]; h + 1];
    dp[0][0] = 0;

    for r in 0..h {
        for m in 0..l {
            if dp[r][m] < 0 {
                continue;
            }
            dfs(0, w, m, 0, r, &a[r], dp[r][m], &mut dp[r + 1]);
        }
    }
    let res = *dp[h].iter().max().unwrap();
    println!("{}", res);
}
fn dfs(
    col: usize,
    w: usize,
    current: usize,
    next: usize,
    r: usize,
    a: &Vec<usize>,
    max: isize,
    dp_next: &mut Vec<isize>,
) {
    if col == w {
        dp_next[next] = dp_next[next].max(max);
        return;
    }
    if current & (1 << col) != 0 {
        dfs(col + 1, w, current, next, r, a, max, dp_next);
        return;
    }

    dfs(
        col + 1,
        w,
        current,
        next,
        r,
        a,
        max ^ a[col] as isize,
        dp_next,
    );

    if col + 1 < w && current & (1 << (col + 1)) == 0 {
        dfs(col + 2, w, current, next, r, a, max, dp_next);
    }

    dfs(col + 1, w, current, next | (1 << col), r, a, max, dp_next);
}
