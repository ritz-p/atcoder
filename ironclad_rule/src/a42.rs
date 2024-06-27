use proconio::input;

fn main(){
    input!{
        n: usize,
        k: usize,
        ab: [(usize,usize);n]
    };

    let mut res = 0;
    for a in 1..100{
        for b in 1..100{
            let count = get_score(a, b, &ab, k);
            res = res.max(count);
        }
    }

    println!("{}",res);
}

fn get_score(a: usize,b:usize,ab: &Vec<(usize,usize)>,k:usize) -> usize{
    let mut count = 0;
    
    for (a2,b2) in ab{
        if a <= *a2 && a2 <= &(a+k) && b <= *b2 && b2 <= &(b+k){
            count += 1;
        }
    }
    count
}