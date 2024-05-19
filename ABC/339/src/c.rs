use proconio::input;

fn main(){
    input!{
        n: usize,
        a: [isize;n]
    };
    let mut mini:isize = 0;
    let mut total:isize = 0;
    for i in 0..n{
        total += a[i];
        mini = mini.min(total);
    }
    println!("{}",total - mini);
}
