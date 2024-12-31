use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let m = 200001;
    let mut v = vec![0; m];
    for e in &a {
        v[*e] += 1;
    }
    let mut res = 0;

    for i in 1..m {
        let mut r = 1;
        while i * r < m {
            res += v[i] * v[r] * v[i * r];
            r += 1;
        }
    }

    println!("{}", res);
}
