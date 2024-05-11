use proconio::input;

const N: usize = 100_000_000;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut v = vec![0; n + 1];
    for (i, &x) in a.iter().enumerate() {
        v[i + 1] = v[i] + x;
    }
    let mut res = 0;
    for (i, &x) in a.iter().enumerate() {
        res += v[n] - v[i + 1] + (n - i - 1) * x;
        let j = a.partition_point(|&y| x + y < N);
        if j <= i {
            res -= (n - i - 1) * N;
        } else {
            res -= (n - j) * N;
        }
    }
    println!("{}",res);
}
