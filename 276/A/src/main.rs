use proconio::input;
use proconio::marker::Chars;
use std::convert::TryInto;

fn main(){
    input!{
        str: Chars,
    };
    let mut res:i32 = -1;
    for i in 0..str.len(){
        if str[i] == 'a'{
            res = (i+1).try_into().unwrap();
        }
    }
    println!("{}",res);
}