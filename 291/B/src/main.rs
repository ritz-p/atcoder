use proconio::input;

fn main(){
    input!{
        n: usize,
        mut judges: [usize;n*5]
    };
    judges.sort_unstable();
    let mut total = 0;
    for i in n..4*n{
        total += judges[i];
    }
    let res:f64 = total as f64 / (n*3) as f64;

    println!("{}",res);
}