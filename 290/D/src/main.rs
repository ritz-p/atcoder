use proconio::input;
use num_integer::gcd;
/*
A,B を正整数とし、A と B の最大公約数を g とおく。A=ag,B=bg と表されるとき、a 個の数 0,BmodA,2BmodA,…,(a−1)BmodA の中には、0,g,2g,…,(a−1)g (すなわち 0 以上 A 未満の g の倍数) がちょうど 1 回ずつ現れる。
*/
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