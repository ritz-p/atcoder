use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize;n],
        b: [usize;q]
    };
    a.sort();
    let mut v = vec![0; n + 1];
    for i in 0..n {
        v[i + 1] = v[i] + a[i];
    }

    let max = a[n - 1];

    for bq in b {
        if max < bq {
            println!("-1");
            continue;
        }
        let p = a.partition_point(|&x| x < bq);
        let t = v[p] + (n - p) * (bq - 1);
        let x = t + 1;
        println!("{}", x);
    }
}
