use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars,
    };
    let mut c = 0;
    for i in s{
        if i == '|'{
            c += 1;
        }else{
            if c != 1{
                print!("{}",i);
            }
        }
    }
    println!();
}
