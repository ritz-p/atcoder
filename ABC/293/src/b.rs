use proconio::input;

fn main(){
    input!{
        n:usize,
        p: [usize;n]   
    };
    let mut called = vec![false;n];
    let mut not_called = vec![];
    for i in 0..n{
        if !called[i]{
            called[p[i]-1] = true; 
        }
    }
    for i in 0..n{
        if !called[i]{
            not_called.push(i+1);
        }
    }
    not_called.sort();
    println!("{}",not_called.len());
    for i in 0..not_called.len(){
        print!("{} ",not_called[i]);
    }
}