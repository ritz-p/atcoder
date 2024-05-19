use proconio::input;
use proconio::marker;
fn main(){
    input!{
        h: usize,
        w: usize,
        arr: [marker::Chars;h]
    };
    let mut count = 0;
    for i in 0..h{
        for j in 0..w{
            if arr[i][j] == '#'{
                count += 1;
            }
        }
    }
    println!("{}",count);
}