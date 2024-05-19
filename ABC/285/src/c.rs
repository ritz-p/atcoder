use proconio::input;

fn main(){
    input!{
        s: String,
    };
    let mut res = 0;
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for i in 0..s.len(){
        for j in 0..26{
            if s.chars().nth(i) == alphabet.chars().nth(j){
                let mut num = j+1;
                for k in 0..s.len()-i-1{
                    num *= 26;
                }
                res += num;
                break;
            }
        }
    
    }
    println!("{}",res);
}
