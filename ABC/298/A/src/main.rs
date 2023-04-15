use proconio::input;
fn main(){
    input!{
        n: usize,
        s: String
    };
    let mut o = 0;
    for i in 0..n{
        if s.chars().nth(i).unwrap() == 'x'{
            println!("No");
            return;
        }
        if s.chars().nth(i).unwrap() == 'o'{
            o += 1;
        }
    }
    if o > 0{
        println!("Yes");
    }else{
        println!("No");
    }
}
