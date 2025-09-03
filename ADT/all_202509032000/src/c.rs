use proconio::input;
fn main() {
    input! {
        mut a: usize,
        b: usize,
        k: usize,
    };
    let mut current = 0;
    while b > a {
        current += 1;
        a *= k;
    }
    println!("{}", current);
}
