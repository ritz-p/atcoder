use proconio::input;
fn main(){
    input!{
        n: usize,
        mut a: [usize;n]
    };
    a.sort();
    let mut sum:usize = a.iter().sum();

    let mut res = 0;
    for i in 0..n-1{
        sum -= a[i];
        res += sum / a[i];
        res -= sum % a[i];
    }

    println!("{}",res);
}
