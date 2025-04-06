use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize;n],
        lr: [(usize,usize);q]
    };
    let mut sums = vec![0; n + 1];
    for i in 0..n {
        sums[i + 1] = a[i] + sums[i];
    }
    for (l, r) in &lr {
        println!("{}", sums[*r] - sums[l - 1]);
    }
}
