use std::fs::File;
use std::io::{self, BufRead};

fn count_clusters(points: &Vec<Node>) -> usize {
    let mut visited: Vec<bool> = vec![false; points.len()];
    let mut cluster_count = 0;

    for i in 0..points.len() {
        if !visited[i] {
            cluster_count += 1;
            let mut stack: Vec<usize> = Vec::new();
            stack.push(i);
            while let Some(node_id) = stack.pop() {
                if !visited[node_id] {
                    visited[node_id] = true;
                    for &neighbor_id in &points[node_id].neighbors {
                        if !visited[neighbor_id] {
                            stack.push(neighbor_id);
                        }
                    }
                }
            }
        }
    }

    cluster_count
}

pub struct Node {
    pub id: usize,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub neighbors: Vec<usize>, // store neighbor node IDs
}

impl Node {
    pub fn new(id: usize, x: u32, y: u32, z: u32) -> Self {
        Self {
            id,
            x,
            y,
            z,
            neighbors: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor_id: usize) {
        self.neighbors.push(neighbor_id);
    }
}

fn main() -> io::Result<()> {
    let file = File::open("day8.txt")?;
    let reader = io::BufReader::new(file);

    let max_edges: usize = 1000;

    // populate points from input
    let mut points: Vec<Node> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let coords: Vec<u32> = line
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let id = points.len();
        points.push(Node::new(id, coords[0], coords[1], coords[2]));
    }

    // find all distances in sorted order
    let mut distances: Vec<(usize, usize, u32)> = Vec::new(); // (point1_id, point2_id, distance)
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dx = points[i].x.abs_diff(points[j].x) as u64;
            let dy = points[i].y.abs_diff(points[j].y) as u64;
            let dz = points[i].z.abs_diff(points[j].z) as u64;


            // squared distance as integer
            let dist2 = dx * dx + dy * dy + dz * dz;

            // integer square root → always gives integer distance
            let dist = dist2.isqrt() as u32;

            distances.push((i, j, dist));
        }
    }

    distances.sort_by_key(|k| k.2);

    // starting from smallest distance, connect points until 1 cluster
    let mut result = 1;
    for (i, j, _) in distances {
        points[i].add_neighbor(j);
        points[j].add_neighbor(i);
        if count_clusters(&points) == 1 {
            result *= (points[i].x) * (points[j].x);
            break;
        }
    }

    println!("Result: {}", result);

    // // find clusters using DFS
    // let mut visited: Vec<bool> = vec![false; points.len()];
    // let mut clusters: Vec<Vec<usize>> = Vec::new();
    // for i in 0..points.len() {
    //     if !visited[i] {
    //         let mut stack: Vec<usize> = Vec::new();
    //         let mut cluster: Vec<usize> = Vec::new();
    //         stack.push(i);
    //         while let Some(node_id) = stack.pop() {
    //             if !visited[node_id] {
    //                 visited[node_id] = true;
    //                 cluster.push(node_id);
    //                 for &neighbor_id in &points[node_id].neighbors {
    //                     if !visited[neighbor_id] {
    //                         stack.push(neighbor_id);
    //                     }
    //                 }
    //             }
    //         }
    //         clusters.push(cluster);
    //     }
    // }

    // println!("Clusters: {:?}", clusters);
    // println!("Product of top 3 largest clusters sizes: {}", {
    //     let mut cluster_sizes: Vec<usize> = clusters.iter().map(|c| c.len()).collect();
    //     cluster_sizes.sort_unstable_by(|a, b| b.cmp(a));
    //     cluster_sizes.iter().take(3).product::<usize>()
    // });

    Ok(())
}