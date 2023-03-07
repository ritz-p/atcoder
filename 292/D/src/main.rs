use proconio::input;
use proconio::marker::Usize1;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    m: usize,
    uv: [(Usize1, Usize1); m]
  }
  // union find について調べる
  let mut uf = UnionFind::<usize>::new(n);
  let mut ans = false;
  
  for (u, v) in uv.iter() {
    uf.union(*u, *v); 
  }
//   println!("{:?}",uf);
  
  let label = uf.into_labeling();
  
  let mut nodes = vec![0; n];
  for i in 0..n {
    nodes[label[i]] += 1;
  }
  
  let mut edges = vec![0; n];
  for (u, _) in uv.iter() {
    edges[label[*u]] += 1;
  }
  
  if nodes == edges {
    ans = true;
  }
  
  if ans == true {
    println!("Yes");
  } else if ans == false {
    println!("No");
  }
}