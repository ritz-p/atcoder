use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars
    };
    if x[0] == x[1] && x[1] == x[2] && x[2] == x[3] {
        println!("Weak");
    } else {
        let mut f = true;
        for i in 1..=3 {
            let current = (x[i - 1] as usize - 47) % 10;
            if char::from_digit(current as u32, 10).unwrap() != x[i] {
                f = false;
                break;
            }
        }
        if f {
            println!("Weak");
        } else {
            println!("Strong");
        }
    }
}
