use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    if 2 * n - 2 > k {
        println!("No");
    } else {
        if k % 2 == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
