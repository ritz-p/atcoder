use proconio::input;
fn main() {
    input! {
        a: u32,
        b: u32,
    };
    println!("{}", 32_usize.pow(a - b));
}
