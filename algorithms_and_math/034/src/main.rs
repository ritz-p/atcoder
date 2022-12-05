use proconio::input;

fn main(){
    input!{
        n: usize,
        mut points: [(f64,f64);n],
    };
    // println!("{:?}",points);
    let mut distance:Vec<isize> = vec![];
    let mut min:f64 = 2_f64.powf(60.0);
    for i in 0..n-1{
        for j in i+1..n{
            let x = points[j].0-points[i].0;
            let y = points[j].1-points[i].1;
            let mut res = x.powi(2) + y.powi(2);
            if min > res{
                min = res;
            }
        }
    }
    println!("{}",min.sqrt());
    // println!("{:?}",distance);
}