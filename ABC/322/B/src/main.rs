use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    };
    let mut first = true;
    let mut last = true;
    for i in 0..n{
        if s[i] != t[i]{
            first = false;
        }
        if s[i] != t[m-n+i]{
            last = false;
        }
    }

    if first && last{
        println!("0");
    }else if first && !last{
        println!("1");
    }else if !first && last{
        println!("2");
    }else{
        println!("3");
    }
}
