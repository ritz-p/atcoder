use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
        d: usize,
        lr: [(usize,usize);d]
    };
    let mut p = vec![0; n + 2];
    let mut q = vec![0; n + 2];

    for i in 1..=n {
        p[i] = p[i - 1].max(a[i - 1]);
    }

    for i in (1..=n).rev() {
        q[i] = q[i + 1].max(a[i - 1]);
    }

    for (l, r) in &lr {
        println!("{}", p[l - 1].max(q[*r + 1]));
    }
}
