use proconio::input;

fn main(){
    input!{
        n: usize,
    };
    let mut res = 1;
    for i in 1..=n{
        res *= i;
    }
    println!("{}",res);
}