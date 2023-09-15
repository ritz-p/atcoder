use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        mut l: [usize;n]
    };
    for i in 0..n{
        l[i] += 1;
    }
    let mut upper:usize = l.iter().sum::<usize>();
    let mut lower:usize = *l.iter().max().unwrap() - 1;
    while lower + 1 < upper{
        let middle = (lower + upper) / 2;
        let mut rows = 1;
        let mut length = 0;
        for i in 0..n{
            length += l[i];
            if length > middle{
                rows += 1;
                length = l[i];
            }
        }

        if rows > m{
            lower = middle;
        }else{
            upper = middle;
        }
    }

    println!("{}",upper-1);
}
