use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars,
        t: Chars,
    };
    let mut res = vec![];
    let mut current = 0;
    for i in 0..s.len(){
        loop{
            if s[i] == t[current]{
                res.push(current+1);
                current += 1;
                break;
            }
            current += 1;
        }
    }
    println!("{}",res.iter().join(" "));
}
