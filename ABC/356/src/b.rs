use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        a: [usize;m],
        x: [[usize;m];n],
    };
    let mut b = vec![0;m];
    for i in 0..n{
        for j in 0..m{
            b[j] += x[i][j];
        }
    }
    for i in 0..m{
        if a[i] > b[i]{
            println!("No");
            return;
        }
    }
    println!("Yes");
}
