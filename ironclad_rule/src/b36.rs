use proconio::input;

fn main() {
    input! {
        _n: usize,
        k: usize,
        a: String
    };
    let mut one = 0;
    for c in a.chars() {
        if c == '1' {
            one += 1;
        }
    }
    if k % 2 == one % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
