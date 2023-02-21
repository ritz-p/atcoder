use proconio::input;
use num_integer::gcd;

fn main(){
    input!{
        t: usize,
        tests: [(usize,usize,usize);t],
    };
    for &(n,d,k) in &tests{
        let k = k - 1;
        let a = n / gcd(n,d);
        println!("{}",d * k % n + k / a);
    }
}