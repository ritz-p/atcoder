use proconio::input;

fn main(){
    input!{
        x: isize
    };
    if x % 10 == 0{
        println!("{}",x / 10);
    }else if x % 10 < 0{
        println!("{}",x / 10);
    }else if x % 10 > 0{
        println!("{}",x / 10 + 1);
    }
}
