use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize;n]
    };
    if a.iter().any(|e| *e == x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
