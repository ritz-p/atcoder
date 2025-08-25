use proconio::input;
fn main() {
    input! {
        x: usize,
        y: usize,
    };
    if (x + y) % 12 != 0 {
        println!("{}", (x + y) % 12);
    } else {
        println!("12");
    }
}
