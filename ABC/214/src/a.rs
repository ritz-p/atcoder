use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n >= 1 && n <= 125 {
        println!("4");
    } else if n >= 126 && n <= 211 {
        println!("6");
    } else {
        println!("8");
    }
}
