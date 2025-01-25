use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize;n],
        x: [usize;q]
    };
    a.sort();
    let mut sums = vec![0; n + 1];
    for i in 0..n {
        sums[i + 1] += sums[i] + a[i];
    }
    for e in &x {
        let b = match a.binary_search(&e) {
            Ok(v) => v,
            Err(v) => v,
        };
        let max = b * e;
        let min = (n - b) * e;
        let res = max - sums[b] + (sums[n] - sums[b]) - min;
        println!("{}", res);
    }
}
