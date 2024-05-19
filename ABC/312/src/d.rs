use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars
    };
    let n = s.len();
    let mut v = vec![vec![0;n+1];n+1];
    let mo = 998244353;
    v[0][0] = 1;
    for i in 0..n{
        for j in 0..n{
            if s[i] != ')'{
                v[i+1][j+1] += v[i][j];
                v[i+1][j+1] %= mo;
            }
            if j != 0 && s[i] != '('{
                v[i+1][j-1] += v[i][j];
                v[i+1][j-1] %= mo;
            }
        }
    }

    println!("{}",v[n][0]);
}
