use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize;n]
    };
    let mut v = vec![false; n + 1];
    for i in 0..k {
        v[p[i]] = true;
    }
    let mut x = p[..k].iter().copied().min().unwrap();
    println!("{}", x);
    for i in k..n {
        if p[i] > x {
            v[x] = false;
            v[p[i]] = true;
        }
        while !v[x] {
            x += 1;
        }
        println!("{}", x);
    }
}
