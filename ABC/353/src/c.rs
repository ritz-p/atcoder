use proconio::input;
use std::collections::BTreeSet;
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut res = 0;
    let m = 100000000;
    let mut bset = BTreeSet::new();
    for i in 0..n{
        res += a[i] * (n-1);
        bset.insert((a[i],i));
    }
    for i in 0..n-1{
        let mut point = bset.range((m-a[i],0)..);
        let mut c = n-i-1;
        if let Some(value) = point.next(){
            c = c.min(n-value.1);
        }else{
            c = 0;
        }
        res -= m * c;
    }

    println!("{}",res);
}
