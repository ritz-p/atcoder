use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        s: Chars,
    };
    let mut count2 = 0;
    let mut str_num:Vec<char> = vec![];
    if s.len() != 8{
        println!("No");
        return 
    }
    if s[0] == '0' || s[0] == '1' || s[0] == '2' || s[0] == '3' || s[0] == '4' || s[0] == '5' || s[0] == '6' || s[0] == '7' || s[0] == '8' || s[0] == '9'{
        println!("No");
        return;
    }
    for i in 1..s.len(){
        if s[i] == '0' || s[i] == '1' || s[i] == '2' || s[i] == '3' || s[i] == '4' || s[i] == '5' || s[i] == '6' || s[i] == '7' || s[i] == '8' || s[i] == '9'{
            count2 += 1;
            str_num.push(s[i]);
        }
    }
    if count2 != 6{
        println!("No");
        return;
    }
    if str_num[0] == '0'{
        println!("No");
        return;
    }
    if s[7] == '0' || s[7] == '1' || s[7] == '2' || s[7] == '3' || s[7] == '4' || s[7] == '5' || s[7] == '6' || s[7] == '7' || s[7] == '8' || s[7] == '9'{
        println!("No");
        return;
    }
    println!("Yes");
}