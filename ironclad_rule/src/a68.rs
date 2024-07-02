use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize,usize,usize);m]
    };
    let mut mf = MaximumFlow::new(n + 1);
    for (a, b, c) in abc {
        mf.add_edge(a, b, c);
    }
    println!("{}", mf.max_flow(1, n));
}

#[derive(Clone, Copy)]
pub struct Edge {
    pub to: usize,
    pub capacity: usize,
    pub reverse: usize,
}

impl Edge {
    pub fn new(to: usize, capacity: usize, reverse: usize) -> Self {
        Self {
            to,
            capacity,
            reverse,
        }
    }
}

pub struct MaximumFlow {
    pub size: usize,
    pub used: Vec<bool>,
    pub graph: Vec<Vec<Edge>>,
}

impl MaximumFlow {
    pub fn new(n: usize) -> Self {
        Self {
            size: n,
            used: vec![false; n + 1],
            graph: vec![Vec::<Edge>::new(); n],
        }
    }

    pub fn add_edge(&mut self, a: usize, b: usize, c: usize) {
        let current_a = self.graph[a].len();
        let current_b = self.graph[b].len();
        self.graph[a].push(Edge::new(b, c, current_b));
        self.graph[b].push(Edge::new(a, 0, current_a));
    }

    pub fn dfs(&mut self, position: usize, goal: usize, flag: usize) -> usize {
        if position == goal {
            return flag;
        }
        self.used[position] = true;

        for i in 0..self.graph[position].len() {
            if self.graph[position][i].capacity == 0 {
                continue;
            }

            if self.used[self.graph[position][i].to] {
                continue;
            }

            let flow = self.dfs(
                self.graph[position][i].to,
                goal,
                flag.min(self.graph[position][i].capacity),
            );

            if flow > 0 {
                self.graph[position][i].capacity -= flow;
                let current = self.graph[position][i];
                self.graph[current.to][current.reverse].capacity += flow;
                return flow;
            }
        }
        0
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut total_flow = 0;
        loop {
            for i in 0..self.size {
                self.used[i] = false;
            }
            let f = self.dfs(s, t, 1000000000);
            if f == 0 {
                break;
            }
            total_flow += f;
        }
        total_flow
    }
}
