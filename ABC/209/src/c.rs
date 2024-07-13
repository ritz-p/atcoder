use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [usize;n]
    };
    let mut res = 1;
    c.sort();
    for i in 0..n {
        res = res * (c[i] - i).max(0) % 1000000007;
    }

    println!("{}", res);
}
