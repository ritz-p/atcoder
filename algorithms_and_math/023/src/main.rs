use proconio::input;

fn main(){
    input!{
        n: i64,
        blue: [i64;n],
        red: [i64;n],
    };

    let mut blue_average = 0.0;
    let mut red_average = 0.0;

    for b in blue.iter(){
        blue_average += (*b as f64) / (n as f64); 
    }

    for r in red.iter(){
        red_average += (*r as f64) / (n as f64);
    }

    let res:f64 = blue_average + red_average;

    println!("{}",res);
}