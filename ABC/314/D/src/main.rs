use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        mut s: Chars,
        q: usize,
    };
    input!{
        v: [(usize,usize,char);q],
    };
    let mut last = 500001;
    for i in 0..q{
        if v[q-1-i].0 == 2 || v[q-1-i].0 == 3{
            last = q-1-i;
            break;
        }
    }
    for i in 0..q{
        if v[i].0 == 1{
            s[v[i].1-1] = v[i].2;
        }
        if i == last{
            if v[i].0 == 2{
                let cs:String = s.iter().collect();
                s = cs.to_lowercase().chars().collect();
            }else if v[i].0 == 3{
                let cs:String = s.iter().collect();
                s = cs.to_uppercase().chars().collect();
            }
        }
    }
    for i in 0..n{
        print!("{}",s[i]);
    }
    println!();
}
