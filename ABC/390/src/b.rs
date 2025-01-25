use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64;n]
    };
    let ratio = a[1] / a[0];
    for i in 1..n - 1 {
        if a[i] * ratio != a[i + 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
