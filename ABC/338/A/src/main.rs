use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars,
    };

    if s[0].is_uppercase(){
        for i in 1..s.len(){
            if s[i].is_uppercase(){
                println!("No");
                return;
            }
        }
    }else{
        println!("No");
        return;
    }
    println!("Yes");
}
