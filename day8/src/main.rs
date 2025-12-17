use std::{cmp::{max, min}, fs};

impl Point {
    fn get_distance(&self, pt: &Point) -> f64 {
        let x_distance = square(&self.x - pt.x);
        let y_distance = square(&self.y - pt.y);
        let z_distance = square(&self.z - pt.z);
        
        let sum = (x_distance + y_distance + z_distance) as f64;
        sum.sqrt()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let points: Vec<Point> = input.lines()
        .map(|line| {
            let coord: Vec<i64> = line
                .split(',')
                .map(|c| c.trim().parse::<i64>().unwrap())
                .collect();

            Point {
                x: coord[0],
                y: coord[1],
                z: coord[2],
            }
        })
        .collect();

    let mut parents: Vec<usize> = (0..points.len()).collect();
    let mut connections: Vec<(f64, usize, usize)> = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = &points[i].get_distance(&points[j]);
            connections.push((*distance, i, j));
        }
    }
    connections.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for (_, i, j) in connections.iter().take(1000) {
        union_merge(&mut parents, *i, *j);
    }

    let mut group_sizes = vec![0; parents.len()];
    for id in 0..points.len() {
        let parent = union_find(&mut parents, id);
        group_sizes[parent] += 1;
    }

    group_sizes.sort();
    let top_group_sizes_product: i32 = group_sizes.iter().rev().take(3).product();
    
    println!();
    for group_size in &group_sizes {
        println!("{group_size}")
    }

    println!("part one, product - {}", top_group_sizes_product);

    let mut parents: Vec<usize> = (0..points.len()).collect();

    let mut last_pair: (usize, usize) = (0, 0);
    let mut merge_count = 0;

    for (_, i, j) in connections.iter() {
        if union_find(&mut parents, *i) == union_find(&mut parents, *j) {
            continue;
        }

        union_merge(&mut parents, *i, *j);
        merge_count += 1;

        if merge_count == points.len() - 1 {
            last_pair = (*i, *j);
        } 
    }

    let (i, j) = last_pair;
    println!("{:?} {:?}", points[i], points[j]);

    let x_product = points[i].x * points[j].x;
    println!("{x_product}");

}

fn union_find(parents: &mut Vec<usize>, i: usize) -> usize {
    if parents[i] != i {
        parents[i] = union_find(parents, parents[i]);
    }

    parents[i]
}

fn union_merge(parents: &mut Vec<usize>, i: usize, j: usize) {
    let i_parent = union_find(parents, i);
    let j_parent = union_find(parents, j);
    
    let min = min(i_parent, j_parent);
    let max = max(i_parent, j_parent);
    
    if i_parent != j_parent {
        parents[max] = min;
    }
}

fn square<T>(x: T) -> T
    where T: std::ops::Mul<Output = T> + Copy,
{
    x * x
}