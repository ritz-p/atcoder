use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        mut b: Chars,
    };
    let mut res = 0;
    for c in 0..b.len(){
        if b[b.len()-1-c] == '1'{
            let p = 2usize.pow(c as u32);
            res += p
        }
    }
    println!("{}",res);
}