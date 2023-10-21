use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        mut s: Chars,
    };

    s.sort();
    let max_value:u128 = 10_i32.pow(n as u32);
    let mut i = 0;
    let mut a = 0;
    while i * i < max_value{
        let mut t:Vec<char> = (i * i).to_string().chars().collect();
        t.resize(n,'0');
        t.sort();
        if t == s{
            a += 1;
        }
        i += 1;
    }
    println!("{}",a);
}
