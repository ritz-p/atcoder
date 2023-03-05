use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars,
    };
    for i in 0..s.len(){
        print!("{}",s[i].to_ascii_uppercase());
    }
}