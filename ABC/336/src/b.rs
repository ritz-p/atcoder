use proconio::input;

fn main(){
    input!{
        n: usize,
    };
    let s = format!("{:b}",n);
    let mut count = 0;
    for i in s.chars().rev(){
        if i == '0'{
            count += 1
        }else{
            break;
        }
    }
    println!("{}",count);
}


