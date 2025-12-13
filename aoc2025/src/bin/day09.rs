use std::fs;
use regex::Regex;
use std::collections::VecDeque;

fn main() {
    let aoc_input = fs::read_to_string("inputs/day09.txt").expect("Document couldn't be read");
    let coordinate_regex = Regex::new(r"(\d+),(\d+)").unwrap();

    let mut coords: Vec<Coordinate> = Vec::new();

    for cap in coordinate_regex.captures_iter(&aoc_input) {
        let x: i64 = cap[1].parse().unwrap();
        let y: i64 = cap[2].parse().unwrap();
        
        coords.push(Coordinate{x, y});
    }

    println!("Part 1: {}", get_largest_area(&coords));
    println!("Part 2: {}", get_largest_area2(&coords));

}

#[derive(Debug)]
struct Coordinate {
    x: i64,
    y: i64,
}

struct GridInfo {
    xs: Vec<i64>,
    ys: Vec<i64>,
    w: usize,
    bad_prefix: Vec<i32>,
}


fn area(c1: &Coordinate, c2: &Coordinate) -> i64 {
    let w: i64 = (c1.x - c2.x).abs() + 1;
    let h: i64 = (c1.y - c2.y).abs() + 1;
    w * h
}

fn get_largest_area(coords: &Vec<Coordinate>) -> i64 {
    let n = coords.len();
    let mut largest_area: i64 = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let a = area(&coords[i], &coords[j]);
            if largest_area < a {
                largest_area = a;
            }
        }
    }
    largest_area
}

fn get_largest_area2(coords: &Vec<Coordinate>) -> i64 {
    let n = coords.len();

    let info = build_bad_prefix_from_loop_compressed(coords);

    let mut best: i64 = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let c1 = &coords[i];
            let c2 = &coords[j];

            if rect_is_fully_allowed(&info, c1, c2) {
                let a = area(c1, c2);
                if a > best {
                    best = a;
                }
            }
        }
    }
    best
}

fn build_axis(values: &Vec<i64>, min_v: i64, max_v: i64) -> Vec<i64> {
    let mut v: Vec<i64> = Vec::new();

    v.push(min_v - 1);
    v.push(max_v + 1);

    for &a in values.iter() {
        v.push(a - 1);
        v.push(a);
        v.push(a + 1);
    }

    v.sort_unstable();
    v.dedup();

    let mut out: Vec<i64> = Vec::new();
    for i in 0..v.len() {
        out.push(v[i]);
        if i + 1 < v.len() {
            let d = v[i + 1] - v[i];
            if d > 1 {
                out.push(v[i] + 1);
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn build_bad_prefix_from_loop_compressed(coords: &Vec<Coordinate>) -> GridInfo {
    let mut min_x = coords[0].x;
    let mut max_x = coords[0].x;
    let mut min_y = coords[0].y;
    let mut max_y = coords[0].y;

    for c in coords.iter() {
        if c.x < min_x {
            min_x = c.x;
        }
        if c.x > max_x {
            max_x = c.x;
        }
        if c.y < min_y {
            min_y = c.y;
        }
        if c.y > max_y {
            max_y = c.y;
        }
    }

    let mut xs_raw: Vec<i64> = Vec::with_capacity(coords.len());
    let mut ys_raw: Vec<i64> = Vec::with_capacity(coords.len());
    for c in coords.iter() {
        xs_raw.push(c.x);
        ys_raw.push(c.y);
    }

    for i in 0..coords.len() {
        let a = &coords[i];
        let b = &coords[(i + 1) % coords.len()];
        xs_raw.push(a.x);
        xs_raw.push(b.x);
        ys_raw.push(a.y);
        ys_raw.push(b.y);
    }

    let xs = build_axis(&xs_raw, min_x, max_x);
    let ys = build_axis(&ys_raw, min_y, max_y);

    let w = xs.len();
    let h = ys.len();

    let mut grid = vec![0u8; w * h];

    let ix = |x: i64, xs: &Vec<i64>| -> usize { xs.binary_search(&x).unwrap() };
    let iy = |y: i64, ys: &Vec<i64>| -> usize { ys.binary_search(&y).unwrap() };

    for i in 0..coords.len() {
        let a = &coords[i];
        let b = &coords[(i + 1) % coords.len()];

        if a.x == b.x {
            let x = a.x;
            let y1 = a.y.min(b.y);
            let y2 = a.y.max(b.y);

            let cx = ix(x, &xs);
            let cy1 = iy(y1, &ys);
            let cy2 = iy(y2, &ys);
            for cy in cy1..=cy2 {
                grid[cy * w + cx] = 1;
            }
        } else {
            let y = a.y;
            let x1 = a.x.min(b.x);
            let x2 = a.x.max(b.x);

            let cy = iy(y, &ys);
            let cx1 = ix(x1, &xs);
            let cx2 = ix(x2, &xs);
            for cx in cx1..=cx2 {
                grid[cy * w + cx] = 1;
            }
        }
    }

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    if grid[0] == 0 {
        grid[0] = 2;
        q.push_back((0, 0));
    }

    while let Some((x, y)) = q.pop_front() {
        if x > 0 {
            let nx = x - 1;
            let idx = y * w + nx;
            if grid[idx] == 0 {
                grid[idx] = 2;
                q.push_back((nx, y));
            }
        }
        if x + 1 < w {
            let nx = x + 1;
            let idx = y * w + nx;
            if grid[idx] == 0 {
                grid[idx] = 2;
                q.push_back((nx, y));
            }
        }
        if y > 0 {
            let ny = y - 1;
            let idx = ny * w + x;
            if grid[idx] == 0 {
                grid[idx] = 2;
                q.push_back((x, ny));
            }
        }
        if y + 1 < h {
            let ny = y + 1;
            let idx = ny * w + x;
            if grid[idx] == 0 {
                grid[idx] = 2;
                q.push_back((x, ny));
            }
        }
    }

    let mut bad = vec![0i32; w * h];
    for i in 0..w * h {
        bad[i] = if grid[i] == 2 { 1 } else { 0 };
    }

    let mut pref = vec![0i32; (w + 1) * (h + 1)];
    for y in 0..h {
        for x in 0..w {
            let v = bad[y * w + x];
            let a = pref[y * (w + 1) + (x + 1)];
            let b = pref[(y + 1) * (w + 1) + x];
            let c = pref[y * (w + 1) + x];
            pref[(y + 1) * (w + 1) + (x + 1)] = a + b - c + v;
        }
    }

    GridInfo {
        xs,
        ys,
        w,
        bad_prefix: pref,
    }
}

fn rect_is_fully_allowed(info: &GridInfo, c1: &Coordinate, c2: &Coordinate) -> bool {
    let x1 = c1.x.min(c2.x);
    let x2 = c1.x.max(c2.x);
    let y1 = c1.y.min(c2.y);
    let y2 = c1.y.max(c2.y);

    let gx1 = info.xs.binary_search(&x1).unwrap();
    let gx2 = info.xs.binary_search(&x2).unwrap();
    let gy1 = info.ys.binary_search(&y1).unwrap();
    let gy2 = info.ys.binary_search(&y2).unwrap();

    rect_sum_bad(info, gx1, gy1, gx2, gy2) == 0
}

fn rect_sum_bad(info: &GridInfo, x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
    let (x1, x2) = (x1.min(x2), x1.max(x2));
    let (y1, y2) = (y1.min(y2), y1.max(y2));

    let w1 = info.w + 1;
    let p = &info.bad_prefix;

    let xa = x1;
    let xb = x2 + 1;
    let ya = y1;
    let yb = y2 + 1;

    p[yb * w1 + xb] - p[ya * w1 + xb] - p[yb * w1 + xa] + p[ya * w1 + xa]
}

