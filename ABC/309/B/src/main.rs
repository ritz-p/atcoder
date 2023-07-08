use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        mut arr: [Chars;n]
    };
    let mut point = arr[0][0];

    for i in 1..n{
        let current = arr[0][i];
        arr[0][i] = point;
        point = current;
    }

    for i in 1..n{
        let current = arr[i][n-1];
        arr[i][n-1] = point;
        point = current;
    }

    for i in 1..n{
        let current = arr[n-1][n-1-i];
        arr[n-1][n-1-i] = point;
        point = current;
    }

    for i in 1..n{
        let current = arr[n-1-i][0];
        arr[n-1-i][0] = point;
        point = current;
    }
    for i in 0..n{
        for j in 0..n{
            print!("{}",arr[i][j]);
        }
        print!("\n");
    }
}
