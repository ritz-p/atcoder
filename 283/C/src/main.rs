use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        mut s: Chars,
    };
    let mut count = s.len();
    let mut minus = 0;
    for i in 1..s.len(){
        if i < s.len()-1{
            if s[i] == '0' && s[i+1] == '0'{
                minus += 1;
                s[i+1] = '1';
            }
        }
    }
    // println!("{}",minus);
    println!("{}",count-minus);
}