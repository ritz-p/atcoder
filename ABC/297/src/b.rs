use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars,
    };
    let mut k = 0;
    let mut q = 0;
    let mut r = 0;
    let mut r1 = 9;
    let mut r2 = 9;
    let mut b = 0;
    let mut b1 = 9;
    let mut b2 = 9;
    let mut n = 0;
    for i in 0..8{
        match s[i]{
            'K' => k = i,
            'Q' => q += 1,
            'R' => if r1 == 9{
                r1 = i;
                r+=1;
            }else{
                r2 = i;
                r+=1;
            },
            'N' => n+=1,
            'B' => if b1 == 9{
                b1 = i;
                b+=1;
            }else{
                b2 = i;
                b+=1;
            },
            _ => {}
        }
    }
    if q != 1{
        println!("No");
        return;
    }
    if n != 2{
        println!("No");
        return;
    }
    if b != 2{
        println!("No");
        return;
    }
    if r != 2{
        println!("No");
        return;
    }
    if k == 0 || k < r1 || r2 < k{
        println!("No");
        return;
    }
    if b1 % 2 == b2 % 2{
        println!("No");
        return;
    }
    println!("Yes");
}
