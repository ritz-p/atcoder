use proconio::input;

fn main(){
    input!{
        a: usize,
        b: usize,
    };
    if b == a+1 && a % 3 != 0{
        println!("Yes");
    }else{
        println!("No");
    }
}
