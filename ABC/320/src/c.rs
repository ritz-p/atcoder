use proconio::input;
use proconio::marker::Chars;
use itertools::Itertools;
use std::char::from_digit;
fn main(){
    input!{
        m: usize,
        s1: Chars,
        s2: Chars,
        s3: Chars,
    };
    let s = vec![s1,s2,s3];
    let mut res = usize::MAX;
    let order = vec![0,1,2];
    for i in 0..10{
        // n が実質 i
        let n = from_digit(i,10).unwrap();
        if s.iter().any(|v| !v.contains(&n)){
            continue;
        }

        for o in order.iter().permutations(3){
            let mut c = 0;
            let mut t = 0;
            loop{
                if s[*o[c]][t%m] == n{
                    c += 1;
                }
                if c == 3{
                    if t < res{
                        res = t;
                    }
                    break
                }
                t += 1;
            }
        }
    }
    if res == usize::MAX{
        println!("-1");
    }else{
        println!("{}",res);
    }
}
