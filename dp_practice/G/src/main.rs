use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        bars: [(usize,usize);m],
    };
    let mut arr = vec![vec![];n];

    for i in 0..n{
        arr[bars[i].0-1].push(bars[i].1-1);
    }

    println!("{:?}",arr);
}