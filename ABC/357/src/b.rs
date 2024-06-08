use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars,
    };
    let mut l = 0;
    let mut u = 0;
    for c in &s{
        if c.is_lowercase(){
            l += 1;
        }else{
            u += 1;
        }
    }
    if l > u{
        for c in s{
            print!("{}",c.to_lowercase());
        }
    }else{
        for c in &s{
            print!("{}",c.to_uppercase());
        }
    }
    println!();
}
