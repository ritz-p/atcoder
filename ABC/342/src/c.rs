use proconio::input;
fn main(){
    input!{
        _n: usize,
        mut s: String,
        q: usize,
    };
    let from = "abcdefghijklmnopqrstuvwxyz".to_string();
    let mut to = "abcdefghijklmnopqrstuvwxyz".to_string();

    for _i in 0..q{
        input!{
            c: String,
            d: String,
        }
        to = to.replace(&c,&d);
    }
    for c in s.chars(){
        for i in 0..26{
            if c == from.chars().nth(i).unwrap(){
                print!("{}",to.chars().nth(i).unwrap());
            }
        }
    }

    println!();
}