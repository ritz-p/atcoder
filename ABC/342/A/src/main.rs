use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars,
    };
    for i in 0..s.len()-2{
        if s[i] != s[i+1] && s[i] != s[i+2]{
            println!("{}",i+1);
            return;
        }else if s[i] != s[i+1] && s[i+1] != s[i+2]{
            println!("{}",i+2);
            return;
        }else if s[i] != s[i+2] && s[i+1] != s[i+2]{
            println!("{}",i+3);
            return;
        }
    }
}
