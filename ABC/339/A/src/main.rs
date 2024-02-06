use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars,
    };
    let mut res_rev:Vec<char> = vec![];
    for i in s.iter().rev(){
        if *i == '.'{
            break;
        }
        res_rev.push(*i);
    }
    for i in res_rev.iter().rev(){
        print!("{}",i);
    }
}
