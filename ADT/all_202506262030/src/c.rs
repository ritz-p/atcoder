use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut v = vec![];
    for i in 0..n {
        v.push((a[i], i));
    }
    v.sort_by(|a, b| a.0.cmp(&b.0));

    println!("{}", v[n - 2].1 + 1);
}
