use proconio::input;

fn main(){
    input!{
        n: usize,
        s: String,
    };
    let mut g = vec![0;n];
    g[0] = 1;
    let mut current = 1;

    for (index,c) in s.chars().enumerate(){
        match c{
            'A' => {
                current += 1;
            },
            'B' => {
                current = 1;
            },
            _ => {}
        }
        g[index + 1] = current;
    }
    current = 1;
    for index in (0..n-1).rev(){
        match s.chars().nth(index){
            Some('A') => {
                current = 1;
            },
            Some('B') => {
                current += 1;
            },
            _ => {}
        }
        g[index] = g[index].max(current);
    }
    let res:usize = g.iter().sum();
    println!("{}",res);
}