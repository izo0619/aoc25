use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

type Point = (i64, i64);
type Edge = (Point, Point);

fn build_loop(edges: &Vec<Edge>) -> Vec<Point> {
    let mut adj: HashMap<Point, Vec<Point>> = HashMap::new();

    // Build adjacency list
    for &(a, b) in edges {
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    }

    // Start at any point
    let start = edges[0].0;
    let mut loop_points = vec![start];
    let mut current = start;
    let mut prev = start;

    loop {
        let neighbors = &adj[&current];
        let next = *neighbors.iter().find(|&&p| p != prev).unwrap_or(&start);

        if next == start {
            break;
        }

        loop_points.push(next);
        prev = current;
        current = next;
    }

    loop_points
}

fn point_on_edge(px: i64, py: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
    let dx = x2 - x1;
    let dy = y2 - y1;

    // Check collinear
    if dx * (py - y1) != dy * (px - x1) {
        return false;
    }

    // Check if within bounding box
    (px >= x1.min(x2) && px <= x1.max(x2)) &&
    (py >= y1.min(y2) && py <= y1.max(y2))
}

fn point_in_polygon_inclusive(point: Point, polygon: &Vec<Point>) -> bool {
    let (px, py) = point;
    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        // First, check if point is on this edge
        if point_on_edge(px, py, x1, y1, x2, y2) {
            return true;
        }

        // Ray-casting: check if edge crosses horizontal ray to the right
        if (y1 > py) != (y2 > py) {
            // Compute intersection x-coordinate using integer division
            // NOTE: use (py - y1) here (vertical offset), not px — px is the test point's x
            let x_intersect = x1 + (py - y1) * (x2 - x1) / (y2 - y1);
            if px < x_intersect {
                inside = !inside;
            }
        }
    }

    inside
}

fn rectangle_inside_polygon(
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
    polygon: &Vec<Point>
) -> bool {
    let corners = [
        (x0, y0),
        (x1, y0),
        (x0, y1),
        (x1, y1),
    ];

    corners.iter().all(|&p| point_in_polygon_inclusive(p, polygon))
}



pub fn is_valid_area(
    segments: &Vec<Edge>,
    loop_points: &Vec<Point>,
    start_x: i64,
    start_y: i64,
    end_x: i64,
    end_y: i64,
) -> bool {
    // check all segments to see if any gp through the area
    // println!("Checking area: ({}, {}) to ({}, {})", start_x, start_y, end_x, end_y);
    for segment in segments {
        let ((x1, y1), (x2, y2)) = *segment;
        // check if segment is vertical
        if x1 == x2 {
            // if the vertical line x is inside the rectangle x-range
            if x1 > start_x && x1 < end_x {
                // normalize y-range for the segment and check for interval overlap with rectangle's y-range
                let seg_min_y = y1.min(y2);
                let seg_max_y = y1.max(y2);
                if seg_max_y > start_y && seg_min_y < end_y {
                    return false;
                }
            }
        } else if y1 == y2 {
            // horizontal segment: if the horizontal line y is inside the rectangle y-range
            if y1 > start_y && y1 < end_y {
                let seg_min_x = x1.min(x2);
                let seg_max_x = x1.max(x2);
                if seg_max_x > start_x && seg_min_x < end_x {
                    // println!("Segment {:?} crosses area.", segment);
                    return false;
                }
            }
        }
    }
    // check that rectangle is inside closed loop
    return rectangle_inside_polygon(start_x, start_y, end_x, end_y, loop_points)
}


fn main() -> io::Result<()> {
    let file = File::open("day9.txt")?;
    let reader = io::BufReader::new(file);

    // populate points from input
    let mut points: Vec<Vec<i64>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let coords: Vec<i64> = line
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        points.push(coords);
    }
    
    // go through points and fill if in same row or col
    // let mut max_area: i64 = 0;
    let mut segments: Vec<((i64, i64), (i64, i64))> = Vec::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            if points[i][0] == points[j][0] {
                segments.push(((points[i][0], points[i][1]), (points[j][0], points[j][1])));
            } else if points[i][1] == points[j][1] {
                segments.push(((points[i][0], points[i][1]), (points[j][0], points[j][1])));
            }
        }
    }
    println!("Created {} segments.", segments.len());
    // println!("Segments: {:?}", segments);

    // find loop points
    let loop_points = build_loop(&segments);
    // println!("Loop points: {:?}", loop_points); 

    // alternatively, manual implementation:
    // for row in 0..=(max_y+1)  as usize {
    //     let mut col = 0;
    //     while col <= max_x as usize {
    //         if matrix[row][col] == 'X' {
    //             // found start X, look for end X
    //             let start_col = col;
    //             col += 1;
    //             while col <= max_x as usize && matrix[row][col] != 'X' {
    //                 col += 1;
    //             }
    //             if col <= max_x as usize && matrix[row][col] == 'X' {
    //                 // found end X, fill in between
    //                 for fill_col in start_col..=col {
    //                     matrix[row][fill_col] = 'X';
    //                 }
    //             }
    //         } else {
    //             col += 1;
    //         }
    //     }
    // }

    // // print matrix for debugging
    // for row in &matrix {
    //     let line: String = row.iter().collect();
    //     println!("{}", line);
    // }

    // now go through all point pairs to find max area that is all X or #
    let mut max_area: i64 = 0;
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let start_x = points[i][0].min(points[j][0]) as usize;
            let end_x = points[i][0].max(points[j][0]) as usize;
            let start_y = points[i][1].min(points[j][1]) as usize;
            let end_y = points[i][1].max(points[j][1]) as usize;

            let area = ((end_x - start_x + 1) * (end_y - start_y + 1)) as i64;
            if area < max_area {
                continue;
            }

            let is_valid = is_valid_area(&segments, &loop_points, start_x as i64, start_y as i64, end_x as i64, end_y as i64);
            // println!("Area {} from ({}, {}) to ({}, {}) is valid: {}", area, start_x, start_y, end_x, end_y, is_valid);
            if is_valid {
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }
    println!("Max area: {}", max_area);

    Ok(())
}