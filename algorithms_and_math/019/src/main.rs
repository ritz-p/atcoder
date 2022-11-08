use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [usize;n]
    };
    let mut red:i64 = 0;
    let mut yellow:i64 = 0;
    let mut blue:i64 = 0;

    for element in arr.iter(){
        if *element == 1{
            red += 1;
        }else if *element == 2{
            yellow += 1;
        }else if *element == 3{
            blue += 1;
        }
    }
    let mut res = 0;
    res += (red * (red - 1)) / 2;
    res += (yellow * (yellow - 1)) / 2;
    res += (blue * (blue - 1)) / 2;

    println!("{}",res);
}