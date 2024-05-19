use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars
    };
    for i in s{
        if i != 'a' && i != 'e' && i != 'i' && i != 'o' && i != 'u'{
            print!("{}",i);
        }
    }
    println!();
}
