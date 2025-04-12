use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
    };
    if n < k {
        println!("1");
        return;
    }
    let mut v = vec![0; n + 1];
    for i in 0..k {
        v[i] = 1;
    }
    let mut count: usize = 0;
    for i in 0..k {
        count = (count + v[i]) % 1000000000;
    }
    v[k] = count;
    for i in k..n {
        count = (count + v[i]) % 1000000000;
        count = (count + 1000000000 - v[i - k]) % 1000000000;
        v[i + 1] = count;
    }

    println!("{}", v[n]);
}
