use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    if x % 100 == 0 && x >= 100 {
        println!("Yes");
    } else {
        println!("No");
    }
}
