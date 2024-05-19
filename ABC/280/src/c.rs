use proconio::input;
use proconio::marker;

fn main(){
    input!{
        s: marker::Chars,
        t: marker::Chars,
    };
    for i in 0..t.len(){
        if i < t.len()-1{
            if s[i] != t[i]{
                println!("{}",i+1);
                break;
            }
        }else{
            println!("{}",i+1);
        }
    }
}