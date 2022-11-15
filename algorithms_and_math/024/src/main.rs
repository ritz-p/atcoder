use proconio::input;

fn main(){
    input!{
        n: usize,
        hash: [(i64,i64);n],
    };

    let mut res = 0.0;
    for (choice,point) in hash.iter(){
        res += (*point as f64) / (*choice as f64)
    }

    println!("{}",res);
}