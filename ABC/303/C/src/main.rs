use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
// vec より hashset のほうが速いケース
fn main(){
    input!{
        n: usize,
        m: usize,
        mut h: isize,
        k: isize,
        s: Chars,
        mut p: [(isize,isize);m],
    };
    let mut x = 0;
    let mut y = 0;
    let mut res = true;
    let mut hash = HashSet::new();
    for i in p{
        hash.insert(i);
    }
    for i in 0..n{
        if s[i] == 'R'{
            x += 1;
        }else if s[i] == 'L'{
            x -= 1;
        }else if s[i] == 'U'{
            y += 1;
        }else if s[i] == 'D'{
            y -= 1;
        }
        h -= 1;
        if h < 0{
            res = false;
            break;
        }
        if h < k{
            if hash.contains(&(x,y)) {
                hash.remove(&(x,y));
                h = k;
            }
        }
    }
    if res{
        println!("Yes");
    }else{
        println!("No");
    }
}
