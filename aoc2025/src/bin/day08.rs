// These solutions takes reference from: 
// https://github.com/Grazen0/advent-of-code-c/blob/main/src/2025/day_08_1.c and
// https://github.com/Grazen0/advent-of-code-c/blob/main/src/2025/day_08_2.c

use std::fs;
use regex::Regex;

fn main() {
    let aoc_input = fs::read_to_string("inputs/day08.txt").unwrap();
    let coordinate_regex = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();

    let mut coords: Vec<Coordinate> = Vec::new();

    for cap in coordinate_regex.captures_iter(&aoc_input) {
        let x = cap[1].parse::<f64>().unwrap();
        let y = cap[2].parse::<f64>().unwrap();
        let z = cap[3].parse::<f64>().unwrap();
        coords.push(Coordinate { x, y, z });
    }

    let n = coords.len();

    let mut edges: Vec<Edge> = Vec::new();
    edges.reserve(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                from: i,
                to: j,
                dist: distance(&coords[i], &coords[j]),
            });
        }
    }

    order_edges_by_distance(&mut edges);

    const K: usize = 1000;
    let part1 = top3_product_after_k_connections(n, &edges, K);
    println!("The product of the 3 largest circuits is: {}", part1);

    let part2 = x_product_of_last_merge(n, &coords, &edges);
    println!("The product of of the X coordinates of the last connection: {}", part2);
}

fn top3_product_after_k_connections(n: usize, edges: &[Edge], k: usize) -> usize {
    let mut nodes = vec![Node { parent: 0, rank: 0 }; n];
    for i in 0..n {
        djset_make_set(&mut nodes, i);
    }

    for e in edges.iter().take(k) {
        djset_union(&mut nodes, e.from, e.to);
    }

    let mut sizes = vec![0usize; n];
    for i in 0..n {
        let root = djset_find(&mut nodes, i);
        sizes[root] += 1;
    }

    sizes.sort_by(|a, b| b.cmp(a));
    sizes[0] * sizes[1] * sizes[2]
}

fn x_product_of_last_merge(n: usize, coords: &[Coordinate], edges: &[Edge]) -> i64 {
    let mut nodes = vec![Node { parent: 0, rank: 0 }; n];
    for i in 0..n {
        djset_make_set(&mut nodes, i);
    }

    let mut components = n;

    for e in edges {
        let a = e.from;
        let b = e.to;

        let ra = djset_find(&mut nodes, a);
        let rb = djset_find(&mut nodes, b);

        if ra != rb {
            djset_union(&mut nodes, ra, rb);
            components -= 1;

            if components == 1 {
                return (coords[a].x as i64) * (coords[b].x as i64);
            }
        }
    }

    -1
}

#[derive(Debug)]
struct Coordinate {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    dist: f64,
}

#[derive(Clone, Copy, Debug)]
struct Node {
    parent: usize,
    rank: usize,
}

fn distance(c1: &Coordinate, c2: &Coordinate) -> f64 {
    let dx = c1.x - c2.x;
    let dy = c1.y - c2.y;
    let dz = c1.z - c2.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn order_edges_by_distance(edges: &mut Vec<Edge>) {
    edges.sort_by(|a, b| {
        a.dist
            .total_cmp(&b.dist)
            .then_with(|| a.from.cmp(&b.from))
            .then_with(|| a.to.cmp(&b.to))
    });
}

fn djset_make_set(nodes: &mut [Node], x: usize) {
    nodes[x] = Node { parent: x, rank: 0 };
}

fn djset_find(nodes: &mut [Node], x: usize) -> usize {
    if nodes[x].parent == x {
        return x;
    }
    let root = djset_find(nodes, nodes[x].parent);
    nodes[x].parent = root;
    root
}

fn djset_union(nodes: &mut [Node], x: usize, y: usize) {
    let x_root = djset_find(nodes, x);
    let y_root = djset_find(nodes, y);

    if x_root == y_root {
        return;
    }

    if nodes[x_root].rank < nodes[y_root].rank {
        nodes[x_root].parent = y_root;
    } else if nodes[y_root].rank < nodes[x_root].rank {
        nodes[y_root].parent = x_root;
    } else {
        nodes[y_root].parent = x_root;
        nodes[x_root].rank += 1;
    }
}


