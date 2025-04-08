use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize,usize,usize,usize);n]
    };
    let mut sums = vec![vec![0; w + 1]; h + 1];
    let mut x = vec![vec![0; w + 2]; h + 2];

    for (a, b, c, d) in abcd {
        x[a][b] += 1;
        x[a][d + 1] -= 1;
        x[c + 1][b] -= 1;
        x[c + 1][d + 1] += 1;
    }
    for i in 1..h + 1 {
        for j in 1..w + 1 {
            sums[i][j] += sums[i][j - 1] + x[i][j];
        }
    }
    for j in 1..w + 1 {
        for i in 1..h + 1 {
            sums[i][j] += sums[i - 1][j];
        }
    }

    for i in 1..h + 1 {
        println!("{}", sums[i].iter().skip(1).join(" "));
    }
}
