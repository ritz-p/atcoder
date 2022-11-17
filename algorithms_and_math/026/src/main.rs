use proconio::input;

fn main(){
    input!{
        n:usize,
    };
    let mut res = 0.0;
    for i in 1..=n{
        res += (n as f64) / (i as f64);
    }

    println!("{}",res);
}