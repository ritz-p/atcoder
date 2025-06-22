use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut query = vec![];
    query.push((0, String::new()));
    let mut pc = vec![0; n];
    let mut server = 0;

    for _ in 0..q {
        input! {
            t: usize
        };
        match t {
            1 => {
                input! {
                    p: usize
                };
                pc[p - 1] = server;
            }
            2 => {
                input! {
                    p: usize,
                    s: String
                };
                let next = query.len();
                query.push((pc[p - 1], s));
                pc[p - 1] = next;
            }
            3 => {
                input! {
                    p: usize
                };
                server = pc[p - 1];
            }
            _ => {}
        }
    }

    let mut res = Vec::new();
    let mut current = server;
    while current != 0 {
        res.push(&query[current].1);
        current = query[current].0;
    }
    res.reverse();

    for r in res {
        print!("{}", r);
    }

    println!();
}
