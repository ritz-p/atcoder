use proconio::input;
// a*b % c = (a % c) * (b % c)
fn main(){
    input!{
        mut a: u64,
        mut b: u64,
        mut c: u64,
        mut d: u64,
        mut e: u64,
        mut f: u64,
    }
    let MOD = 998244353;
    a %= MOD;
    b %= MOD;
    c %= MOD;
    d %= MOD;
    e %= MOD;
    f %= MOD;
    let abc = a * b % MOD * c % MOD;
    let def = d * e % MOD * f % MOD;

    println!("{}",(abc + MOD - def)%MOD);
}