use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        s: Chars
    };
    for i in 0..n-1{
        if s[i] == 'a'{
            if s[i+1] == 'b'{
                println!("Yes");
                return;
            }
        }
        if s[i] == 'b'{
            if s[i+1] == 'a'{
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
