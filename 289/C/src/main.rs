use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut cs = vec![];
    for _ in 0..m {
        input!{
            c: usize, 
            a: [usize; c]
        }
        cs.push(a);
    }
    let mut ans = 0;
    for bit in 0..(1<<m){
        let mut flag = vec![false; n];
        for j in 0..m{
            if ((bit>>j)&1) == 1{
                for &a in &cs[j]{
                    flag[a-1] = true;
                }
            }
        }
        if flag.iter().all(|f| *f){
            ans += 1;
        }
    }
    println!("{}", ans);
}