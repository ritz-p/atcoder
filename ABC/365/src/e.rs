use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }
    let mut xor = vec![0; n + 1];
    for i in 0..n {
        xor[i + 1] = xor[i] ^ a[i];
    }
    let mut res = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            res += xor[i] ^ xor[j + 1];
        }
    }
    println!("{}", res);
}
