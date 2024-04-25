use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        mut s: Chars,
    };
    let cs:String = s.split_off(3).to_owned().into_iter().collect();
    match cs.parse::<usize>(){
        Ok(num) => {
            if num != 316 && num < 350 && num > 0{
                println!("Yes");
            }else{
                println!("No");
            }
        },
        Err(_) => {
            println!("No");
        }
    }
}
