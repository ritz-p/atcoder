use proconio::input;

fn main(){
    input! {
        n: usize,
    };
    let mut v = vec![0;n+1];
    v[1] = 1;
    let r#mod = 1000000007;
    for i in 2..=n{
        v[i] = (v[i-1] + v[i-2]) % r#mod;
    }
    println!("{}",v[n]);
}