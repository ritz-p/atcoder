use proconio::input;
use proconio::marker::Chars;

fn main(){
    input!{
        n: usize,
        m: usize,
        arr: [Chars;n],
    };
    let mut res = 0;
    for i in 0..n-1{
        for j in i+1..n{
            for k in 0..m{
                if arr[i][k] == 'x' && arr[j][k] == 'x'{
                    break;
                }
                if k == m-1{
                    res += 1;
                }
            }
        }
    }
    println!("{}",res);
}