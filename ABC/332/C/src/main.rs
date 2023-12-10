use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        _n: usize,
        m: usize,
        s: Chars
    };
    let mut logo = 0;
    let mut res = 0;
    let mut plain = m;
    for c in s{
        match c{
            '0' => {
                res = res.max(logo);
                logo = 0;
                plain = m;
            },
            '1' => {
                if plain > 0{
                    plain -= 1;
                }else{
                    logo += 1;
                }
            },
            '2' => {
                logo += 1;
            },
            _ => {}
        }
    }
    res = res.max(logo);
    println!("{}",res);
}
