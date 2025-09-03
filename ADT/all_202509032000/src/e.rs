use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize;n]
    };
    let mut res = 0;
    a.sort();

    for i in 0..n {
        let p = a.partition_point(|&v| v < a[i] + m);
        res = res.max(p - i);
    }

    println!("{}", res);
}
