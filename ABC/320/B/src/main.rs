use proconio::input;

fn main(){
    input!{
        s: String,
    };
    let mut res = 1;
    for i in 0..s.len(){
        for j in i..=s.len(){
            if i == j{
                continue;
            }
            let part: &str = &s[i..j];
            let reverse: &str = &part.chars().rev().collect::<String>();
            if part == reverse{
                if res < j-i{
                    res = j-i;
                }
            }
        }
    }
    println!("{}",res);
}
