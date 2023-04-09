use proconio::input;
    
fn main(){
    input!{
        n: usize,
        arr: [String;n],
    };
    //and, not, that, the, you
    for i in arr{
        if i == "and" || i == "not" || i == "that" || i == "the" || i == "you"{
            println!("Yes");
            return
        }
    }
    println!("No");
}
