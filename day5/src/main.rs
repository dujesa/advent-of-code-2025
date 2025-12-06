use std::{fs};

impl Range {
    fn overlap (&self, overlapping_range: &Range) -> (Range, i64) {
        let overlap_min = self.start.max(overlapping_range.start);
        let overlap_max = self.end.min(overlapping_range.end);
        
        let overlap_start = self.start.min(overlapping_range.start);
        let overlap_end = self.end.max(overlapping_range.end);

        let distance = overlap_max - overlap_min + 1;

        (Range {
            start: overlap_start,
            end: overlap_end
        }, match distance > 0 {
            true => 0,
            false => distance,
        })
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Range {
    start: i64,
    end: i64,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let ingredient_inputs: Vec<&str> = input.split("\n\n").collect();
    
    let fresh_ranges: Vec<&str> = ingredient_inputs[0].lines().collect();
    let available_ids: Vec<i64> = ingredient_inputs[1].lines().map(|id| id.trim().parse().unwrap()).collect();

    let mut available_fresh_ids: Vec<i64> = Vec::new();
    
    for fresh_range in fresh_ranges {
        let (min_string, max_string) = fresh_range.split_once("-").expect("invalid range");
        let min: i64 = min_string.trim().parse().unwrap();
        let max: i64 = max_string.trim().parse().unwrap();

        for &available_id in &available_ids {
            if !available_fresh_ids.contains(&available_id) && available_id >= min && available_id <= max {
                available_fresh_ids.push(available_id);
            }
        }
    }

    println!("part one - {}", available_fresh_ids.len());

    let mut fresh_ranges: Vec<Range> = ingredient_inputs[0].lines().map(|l| {
        let (min, max) = l.split_once("-").expect("invalid range");
        Range {
            start: min.trim().parse::<i64>().expect("invalid start"),
            end: max.trim().parse::<i64>().expect("invalid end")
        }
    }).collect();
    
    fresh_ranges.sort_by_key(|r| r.start);
    
    let mut overlapped_range: Range = fresh_ranges[0].clone();
    let mut overlapped_distance = 0;

    for fresh_range in fresh_ranges {
        let mut distance = 0;

        (overlapped_range, distance) = overlapped_range.overlap(&fresh_range);
        overlapped_distance = overlapped_distance + distance;
    }

    let fresh_ids = overlapped_range.end - overlapped_range.start + overlapped_distance + 1;
    println!("part two - {}", fresh_ids);
}

