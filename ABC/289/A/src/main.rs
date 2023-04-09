use proconio::input;

fn main() {
    input!{
        s: String,
    };
    for i in 0..s.len(){
        if s.chars().nth(i).unwrap() == '0'{
            print!("1");
        }else{
            print!("0");
        }
    }
}
