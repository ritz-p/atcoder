use proconio::input;

fn main(){
    input!{
        n: usize,
        q: usize,
        a: [usize;n],
    }
    let mut sums = vec![0;n+1];
    sums[1] = a[0];
    for i in 2..=n{
        sums[i] = a[i-1]+sums[i-1];
    }
    for _i in 0..q{
        input!{
            mut l: usize,
            r: usize,
        };
        let res = sums[r] - sums[l-1];
        println!("{}",res);
    }
}