use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut res = usize::MAX;
    for i in 0..(1 << n) {
        let mut xor = 0;
        let mut or = 0;
        for j in 0..n {
            or |= a[j];
            if j == n - 1 || (i >> j & 1) != 0 {
                xor ^= or;
                or = 0;
            }
        }
        res = res.min(xor);
    }

    println!("{}", res);
}
