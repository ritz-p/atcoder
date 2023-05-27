use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        s: Chars,
        t: Chars,
    };
    let mut res = true;
    for i in 0..n{
        if s[i] == t[i]{
            continue;
        }

        if s[i] == '1'{
            if t[i] == 'l'{
                continue;
            }else{
                res = false;
            }
        }
        if s[i] == 'l'{
            if t[i] == '1'{
                continue;
            }else{
                res = false;
            }
        }
        if s[i] == 'o'{
            if t[i] == '0'{
                continue;
            }else{
                res = false;
            }
        }
        if s[i] == '0'{
            if t[i] == 'o'{
                continue;
            }else{
                res = false;
            }
        }
        if s[i] != t[i]{
            res = false;
        }
    }
    if res{
        println!("Yes");
    }else {
        println!("No");
    }
}
