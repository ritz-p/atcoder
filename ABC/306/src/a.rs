use proconio::input;

fn main(){
    input!{
        n: usize,
        s: String,
    };
    for i in s.chars(){
        print!("{}{}",i,i);
    }
    print!("\n");
}
