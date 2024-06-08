use proconio::input;
use ac_library::modint::ModInt998244353 as Mint;

fn main() {
    input! {
        n: u64,
    }

    let l = n.to_string().len();
    
    let a = Mint::new(n);
    let r = Mint::new(10).pow(l as u64);
    let one = Mint::new(1);
    println!("{}", a * (r.pow(n as u64) - one) / (r - one));
}