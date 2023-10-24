use proconio::input;

fn main() {
    input!{
        n: usize,
    }
    if n % 2 == 1{
        return;
    }

    for bit in 0..(1<<n){
        let mut s = "".to_string();

        for i in (0..n).rev(){
            if (bit &(1<<i)) == 0{
                s += "(";
            }else{
                s += ")";
            }
        }
        if is_valid(&s){
            println!("{}",s);
        }
    }
}

fn is_valid(s: &String) -> bool{
    let mut score = 0;
    for c in s.chars(){
        if c == '('{
            score += 1;
        }else if c == ')'{
            score -= 1;
        }
        if score < 0{
            return false;
        }
    }
    if score == 0{
        return true;
    }
    false
}