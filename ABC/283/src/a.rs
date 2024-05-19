use proconio::input;

fn main(){
    input!{
        a: usize,
        b: usize,
    };
    let mut res = 1;
    for i in 0..b{
        res *= a;
    }
    println!("{}",res);
}