use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize;w];h],
        q: usize,
        abcd: [(usize,usize,usize,usize);q]
    };
    let mut sums = vec![vec![0; w + 1]; h + 1];

    for i in 1..h + 1 {
        for j in 1..w + 1 {
            sums[i][j] = sums[i][j - 1] + x[i - 1][j - 1];
        }
    }
    for j in 1..w + 1 {
        for i in 1..h + 1 {
            sums[i][j] = sums[i - 1][j] + sums[i][j];
        }
    }
    for (a, b, c, d) in abcd {
        let res = sums[c][d] + sums[a - 1][b - 1] - sums[c][b - 1] - sums[a - 1][d];
        println!("{}", res);
    }
}
