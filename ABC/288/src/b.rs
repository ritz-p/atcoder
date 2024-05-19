use proconio::input;

fn main(){
    input!{
        n: usize,
        k: usize,
        mut arr: [String;n],
    };
    let mut arr2:Vec<&str> = vec![];
    for i in 0..k{
        arr2.push(&arr[i]);
    }
    arr2.sort();
    for i in 0..arr2.len(){
        println!("{}",arr2[i]);
    }
}