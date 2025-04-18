use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize,usize);n],
        q: usize,
        abcd: [(usize,usize,usize,usize);q]
    };
    let mut s = vec![vec![0; 1501]; 1501];
    let mut sums = vec![vec![0; 1501]; 1501];

    for (x, y) in &xy {
        s[*x][*y] += 1;
    }

    for i in 1..=1500 {
        for j in 1..=1500 {
            sums[i][j] = sums[i - 1][j] + s[i][j];
        }
    }

    for i in 1..=1500 {
        for j in 1..=1500 {
            sums[i][j] += sums[i][j - 1];
        }
    }

    for (a, b, c, d) in &abcd {
        println!(
            "{}",
            sums[*c][*d] - sums[*c][b - 1] - sums[a - 1][*d] + sums[a - 1][b - 1]
        );
    }
}
