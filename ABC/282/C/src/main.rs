use proconio::input;
use proconio::marker::Chars;

fn main(){
    input!{
        n: usize,
        mut s: Chars,
    }
    let mut places = vec![];
    for i in 0..n{
        if s[i] == ','{
            s[i] = '.';
        }
    }
    for i in 0..n{
        if s[i] == '"'{
            places.push(i);
        }
    }
    let mut max = places.len();
    if max % 2 == 1{
        max -= 1;
    }
    for i in 0..max{
        if i % 2 == 0{
            for j in places[i]..places[i+1]{
                if s[j] == '.'{
                    s[j] = ',';
                }
            }
        }
    }
    for i in 0..n{
        print!("{}",s[i]);
    }
}