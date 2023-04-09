use proconio::input;

fn main(){
    input!{
        str: String,
    };
    let mut count = 0;
    for i in str.chars(){
        if i == 'w'{
            count += 2;
        }else if i == 'v'{
            count += 1;
        }
    }
    println!("{}",count);
}