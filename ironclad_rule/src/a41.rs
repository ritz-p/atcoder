use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        s: Chars,
    };

    let mut res = false;
    for i in 1..n-1{
        if s[i] == s[i-1] && s[i] == s[i+1]{
            res = true;
            break;
        }
    }

    if res{
        println!("Yes");
    }else{
        println!("No");
    }
}