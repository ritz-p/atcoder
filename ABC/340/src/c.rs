use proconio::input;
fn main(){
    input!{
        n: usize
    };
    let mut dig = 0;
    let mut current = 1;
    let mut a = vec![1];
    while current <= n{
        current *= 2;
        dig += 1;
        a.push(current);
    }
    current /= 2;
    a.pop();
    let mut total = a[a.len()-1] * (dig-1);
    let remain = n - current;
    total += remain * (dig + 1);
    println!("{}",total);
}