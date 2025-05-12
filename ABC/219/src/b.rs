use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [String;3],
        t: Chars
    };

    for c in t {
        match c {
            '1' => {
                print!("{}", s[0])
            }
            '2' => {
                print!("{}", s[1]);
            }
            '3' => {
                print!("{}", s[2]);
            }
            _ => {}
        }
    }
    println!();
}
