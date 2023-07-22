use proconio::input;
use proconio::marker::Chars;

fn main(){
    input!{
        n: usize,
        d: usize,
        arr: [Chars;n]
    };
    let mut count = 0;
    let mut con = 0;
    for j in 0..d{
        for i in 0..n{
            if arr[i][j] != 'o'{
                con = 0;
                break;
            }
            if arr[i][j] == 'o' && i == n-1{
                con += 1;
                if con > count{
                    count = con;
                }
            }
        }
    }
    println!("{}",count);
}
