use proconio::input;

fn main(){
    input!{
        n: usize,
    };
    let b = format!("{:b}",n);
    let mut res = 0;
    for c in b.chars().rev(){
        if c == '0'{
            res += 1;
        }else{
            break;
        }
    }
    println!("{}",res);
}
