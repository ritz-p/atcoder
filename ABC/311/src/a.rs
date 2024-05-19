use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        arr: Chars
    };

    let mut a = false;
    let mut b = false;
    let mut c = false;

    for i in 0..n{
        if arr[i] == 'A'{
            a = true;
        }
        if arr[i] == 'B'{
            b = true;
        }
        if arr[i] == 'C'{
            c = true;
        }
        if a && b && c{
            println!("{}",i+1);
            break;
        }
    }
}
