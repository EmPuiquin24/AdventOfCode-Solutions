use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    name_to_id: HashMap<String, usize>,
    id_to_name: Vec<String>,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            name_to_id: HashMap::new(),
            id_to_name: Vec::new(),
            adj: Vec::new(),
        }
    }

    fn get_or_add_node(&mut self, name: &str) -> usize {
        if let Some(&id) = self.name_to_id.get(name) {
            return id;
        }

        let id = self.id_to_name.len();
        self.name_to_id.insert(name.to_string(), id);
        self.id_to_name.push(name.to_string());
        self.adj.push(Vec::new());
        id
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        let u = self.get_or_add_node(from);
        let v = self.get_or_add_node(to);

        if !self.adj[u].contains(&v) {
            self.adj[u].push(v);
        }
    }

    fn get_node_id(&self, name: &str) -> usize {
        self.name_to_id[name]
    }

    fn neighbors(&self, id: usize) -> &[usize] {
        &self.adj[id]
    }

    fn len(&self) -> usize {
        self.id_to_name.len()
    }
}

fn compute_can_reach_target(g: &Graph, target: usize) -> Vec<bool> {
    let n = g.len();
    let mut rev_adj: Vec<Vec<usize>> = vec![Vec::new(); n];

    for u in 0..n {
        for &v in &g.adj[u] {
            rev_adj[v].push(u);
        }
    }

    let mut can = vec![false; n];
    let mut stack = vec![target];
    can[target] = true;

    while let Some(v) = stack.pop() {
        for &u in &rev_adj[v] {
            if !can[u] {
                can[u] = true;
                stack.push(u);
            }
        }
    }

    can
}

fn count_all_paths_dp(g: &Graph, start_name: &str, target_name: &str) -> u64 {
    let start = g.get_node_id(start_name);
    let target = g.get_node_id(target_name);

    let n = g.len();
    let can_reach = compute_can_reach_target(g, target);
    let mut memo: Vec<Option<u64>> = vec![None; n];

    fn dp(
        u: usize,
        target: usize,
        g: &Graph,
        can_reach: &[bool],
        memo: &mut [Option<u64>],
    ) -> u64 {
        if !can_reach[u] {
            return 0;
        }

        if let Some(val) = memo[u] {
            return val;
        }

        if u == target {
            memo[u] = Some(1);
            return 1;
        }

        let mut total = 0u64;
        for &v in g.neighbors(u) {
            total += dp(v, target, g, can_reach, memo);
        }

        memo[u] = Some(total);
        total
    }

    dp(start, target, g, &can_reach, &mut memo)
}

fn count_paths_with_dac_and_fft_dp(g: &Graph, start_name: &str, target_name: &str) -> u64 {
    let start = g.get_node_id(start_name);
    let target = g.get_node_id(target_name);

    let dac_id = g.get_node_id("dac");
    let fft_id = g.get_node_id("fft");

    let can_reach = compute_can_reach_target(g, target);
    let mut memo: HashMap<(usize, bool, bool), u64> = HashMap::new();

    fn dp(
        u: usize,
        target: usize,
        dac_id: usize,
        fft_id: usize,
        saw_dac: bool,
        saw_fft: bool,
        g: &Graph,
        can_reach: &[bool],
        memo: &mut HashMap<(usize, bool, bool), u64>,
    ) -> u64 {
        if !can_reach[u] {
            return 0;
        }

        let mut saw_dac = saw_dac;
        let mut saw_fft = saw_fft;

        if u == dac_id {
            saw_dac = true;
        }
        if u == fft_id {
            saw_fft = true;
        }

        if u == target {
            return if saw_dac && saw_fft { 1 } else { 0 };
        }

        let key = (u, saw_dac, saw_fft);
        if let Some(&ans) = memo.get(&key) {
            return ans;
        }

        let mut total = 0u64;
        for &v in g.neighbors(u) {
            total += dp(
                v, target, dac_id, fft_id, saw_dac, saw_fft, g, can_reach, memo,
            );
        }

        memo.insert(key, total);
        total
    }

    dp(
        start,
        target,
        dac_id,
        fft_id,
        false,
        false,
        g,
        &can_reach,
        &mut memo,
    )
}

fn read_graph_from_file(path: &str) -> Graph {
    let input = fs::read_to_string(path).unwrap();
    let mut g = Graph::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some((left, right)) = line.split_once(':') {
            let src = left.trim();
            let rhs = right.trim();

            g.get_or_add_node(src);

            if rhs.is_empty() {
                continue;
            }

            for dst_raw in rhs.split_whitespace() {
                let dst = dst_raw.trim();
                if dst.is_empty() {
                    continue;
                }
                g.add_edge(src, dst);
            }
        }
    }

    g
}

fn main() {
    let aoc_graph = read_graph_from_file("inputs/day11.txt");

    let paths_p1 = count_all_paths_dp(&aoc_graph, "you", "out");
    println!("Ways from you to out (p1): {}", paths_p1);

    let paths_p2 = count_paths_with_dac_and_fft_dp(&aoc_graph, "svr", "out");
    println!("Ways from svr to out with dac and fft (p2): {}", paths_p2);
}


