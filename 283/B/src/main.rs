use proconio::input;

fn main(){
    input!{
        n: usize,
        mut arr: [usize;n],
        m: usize,
    };
    let mut queries = vec![vec![];m];
    for i in 0..m{
        input!{
            q: usize,
        };
        if q == 1{
            input!{
                k:usize,
                x:usize,
            };
            queries[i].push(q);
            queries[i].push(k);
            queries[i].push(x);
        }else{
            input!{
                k:usize,
            };
            queries[i].push(q);
            queries[i].push(k);
        }
    }
    // println!("{:?}",queries);
    for i in 0..m{
        if queries[i][0] == 1{
            arr[queries[i][1]-1] = queries[i][2];
        }else{
            println!("{}",arr[queries[i][1]-1]);
        }
    }
}