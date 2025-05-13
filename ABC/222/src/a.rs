use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars
    };
    for _i in 0..4 - s.len() {
        print!("0");
    }
    println!("{}", s.iter().collect::<String>());
}
