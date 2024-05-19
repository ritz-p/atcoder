use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        mut s: String,
    };
    let res = s.replace("na","nya");
    for i in 0..res.len(){
        print!("{}",res.chars().nth(i).unwrap());
    }
}