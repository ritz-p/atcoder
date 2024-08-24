use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n]
    };
    for i in n - k..n {
        print!("{} ", a[i]);
    }
    for i in 0..n - k {
        print!("{} ", a[i]);
    }
    println!();
}
