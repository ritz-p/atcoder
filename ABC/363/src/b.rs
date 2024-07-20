use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        mut l: [usize;n]
    };


    for i in 0..t{
        if l.iter().filter(|x| **x >= t).count() >= p{
            println!("{}",i);
            return;
        }
        for j in 0..n{
            l[j] += 1;
        }
    }
}
