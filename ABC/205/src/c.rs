use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
    };
    if c % 2 == 0 {
        if a.abs() > b.abs() {
            println!(">");
        } else if a.abs() < b.abs() {
            println!("<");
        } else {
            println!("=");
        }
    } else {
        if a < 0 && b < 0 {
            if a.abs() < b.abs() {
                println!(">");
            } else if a.abs() > b.abs() {
                println!("<");
            } else {
                println!("=");
            }
        } else {
            if a > b {
                println!(">");
            } else if a < b {
                println!("<");
            } else {
                println!("=");
            }
        }
    }
}
