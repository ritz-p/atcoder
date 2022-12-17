use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n:usize,
    };
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for i in 0..n{
        print!("{}",alphabet.chars().nth(i).unwrap());
    }
}