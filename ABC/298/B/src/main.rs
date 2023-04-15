use proconio::input;
    
fn main(){
    input!{
        n: usize,
        mut a: [[usize; n];n],
        b: [[usize; n];n]
    };
    let mut flag = false;
    for i in 0..4{
        for j in 0..n{
            for k in 0..n{
                if a[j][k] == 1 && b[j][k] != 1{
                    flag = true;
                    break;
                }
            }
            if flag{
                break;
            }
            if j == n-1{
                println!("Yes");
                return;
            }
        }
        for j in 0..n/2{
            for k in j..n-j-1 {
                let tmp = a[j][k];
                a[j][k] = a[n-1-k][j];
                a[n-1-k][j] = a[n-1-j][n-1-k];
                a[n-1-j][n-1-k] = a[k][n-1-j];
                a[k][n - 1 - j] = tmp;
            }
        }
        // println!("{:?}",a);
        flag = false;
    }
    println!("No");
}
