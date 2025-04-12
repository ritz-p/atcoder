use proconio::input;
fn main() {
    input! {
        n: usize,
    };
    if n >= 200 && n <= 299 {
        println!("Success");
    } else {
        println!("Failure")
    }
}
