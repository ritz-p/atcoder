use proconio::input;

fn main(){
    input!{
        n: usize,
        mut a: [usize;n]
    };
    a.sort();
    let s = a.iter().sum::<usize>();
    let b = s / n;
    let mut c = vec![b;n];

    for i in 0..s % n{
        c[n-1-i] += 1;
    }

    let mut res = 0;
    

    for i in 0..n{
        let x = a[i];
        let y = c[i];
        res += x.max(y) - x.min(y);
    }
    println!("{}",res / 2);
}
