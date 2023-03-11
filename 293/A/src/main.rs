use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        mut s: Chars
    };
    for i in 0..s.len()-1{
        if i % 2 == 0{
            let tmp = s[i];
            s[i] = s[i+1];
            s[i+1] = tmp;
        }
    }
    for i in 0..s.len(){
        print!("{}",s[i]);
    }
}