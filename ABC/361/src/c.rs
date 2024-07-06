use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize;n]
    };
    a.sort();

    let mut res = a[a.len() - k - 1] - a[0];
    for i in 0..=k {
        let low = a[i];
        let high = a[n - k + i - 1];
        res = res.min(high - low);
    }
    println!("{}", res);
}
