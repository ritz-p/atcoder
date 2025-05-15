use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
        x: usize,
    };
    if s < t {
        if x >= s && x < t {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if x >= s || x < t {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
