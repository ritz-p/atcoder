use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars,
        t: Chars,
    };
    let base = "abcdefghijklmnopqrstuvwxyz@";
    let mut sarr = vec![0;27];
    let mut tarr = vec![0;27];
    for i in 0..s.len(){
        for j in 0..27{
            if s[i] == base.chars().nth(j).unwrap(){
                sarr[j]+=1;
            }
            if t[i] == base.chars().nth(j).unwrap(){
                tarr[j]+=1;
            }
        }
    }
    for i in 0..27{
        if sarr[i] == tarr[i]{
            continue;
        }
        if sarr[i] > tarr[i]{
            let diff = sarr[i] - tarr[i];
            if i == 0 || i == 2 || i == 3 || i == 4 || i == 14 || i == 17 || i == 19{
                if tarr[26] >= diff{
                    tarr[i] += diff;
                    tarr[26] -= diff;
                }
            }
        }else if tarr[i] > sarr[i]{
            let diff = tarr[i] - sarr[i];
            if i == 0 || i == 2 || i == 3 || i == 4 || i == 14 || i == 17 || i == 19{
                if sarr[26] >= diff{
                    sarr[i] += diff;
                    sarr[26] -= diff;
                }
            }
        }
        if sarr[i] != tarr[i]{
            println!("No");
            return;
        }
    }
    println!("Yes");
}
