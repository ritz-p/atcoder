use proconio::input;

fn main(){
    input!{
        a: usize,
        mut b: usize,
    };
    let mut current = a;
    let mut res = 1;
    let r#mod = 1000000007;
    for _i in 0..30{
        if b % 2 == 1{
            res *= current;
            res %= r#mod;
        }
        b /= 2;
        current *= current;
        current %= r#mod;
    }
    println!("{}",res);
}