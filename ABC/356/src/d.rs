use proconio::input;

const MOD:usize = 998244353;
fn main(){
    input!{
        n: usize,
        m: usize,
    };
    let bn = format!("{:b}",n);
    let bm = format!("{:b}",m);
    println!("{} {}",bn,bm);
    let mut res = (n & m).count_ones();

    println!("{}",res);
}
