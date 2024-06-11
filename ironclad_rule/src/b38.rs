use proconio::input;

fn main(){
    input!{
        n: usize,
        s: String,
    };
    let mut g = vec![0;n];
    g[0] = 1;
    for (index,c) in s.chars().enumerate(){
        match c{
            'A' => {
                g[index+1] = g[index] + 1;
            },
            'B' => {
                let mut bc = 1;
                let mut current = index+1;
                while s.chars().nth(current).unwrap() == 'B'{
                    bc += 1;
                    current += 1;
                }
                g[index+1] = (g[index] - 1).max(bc);
            },
            _ => {}
        }
    }
    let res:usize = g.iter().sum();
    println!("{:?}",g);
    println!("{}",res);
}