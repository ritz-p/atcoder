use proconio::input;

fn main(){
    let mut v = vec![];
    loop{
        input!{
            n: usize
        };
        v.push(n);
        if n == 0{
            break;
        }
    }
    for i in v.iter().rev(){
        println!("{}",i);
    }
}
