use proconio::input;
use proconio::marker::Chars;

fn main(){
    input!{
        n: usize,
        s: Chars,
    };
    for i in 1..n{
        let mut res = 0;
        for j in 0..n-1{
            if j + i < s.len(){
                if s[j] != s[j+i]{
                    res = j+1;
                }else{
                    break;
                }
            }else{
                break;
            }
        }
        println!("{}",res);
    }
}