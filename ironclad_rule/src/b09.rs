use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize,usize,usize,usize);n]
    };
    let mut sums = vec![vec![0_i32; 1501]; 1501];
    let mut x = vec![vec![0_i32; 1501]; 1501];

    for (a, b, c, d) in abcd {
        x[a][b] += 1;
        x[a][d] -= 1;
        x[c][b] -= 1;
        x[c][d] += 1;
    }

    for i in 0..=1500 {
        sums[i][0] = x[i][0];
        for j in 1..=1500 {
            sums[i][j] += sums[i][j - 1] + x[i][j];
        }
    }

    for i in 1..=1500 {
        for j in 0..=1500 {
            sums[i][j] += sums[i - 1][j];
        }
    }

    let mut res = 0_usize;

    for i in 0..=1500 {
        for j in 0..=1500 {
            if sums[i][j] > 0 {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
