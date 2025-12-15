use std::fs;

#[derive(Debug)]
struct Node {
    x: i64,
    y: i64,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut diagram: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect(); 

    let h = diagram.len();
    if h == 0 {
        eprintln!("Invalid diagram height");
    }

    let w = diagram[0].len();

    let start_column = diagram[0].iter().position(|&b| b == b'S');
    if start_column.is_none() {
        eprintln!("error in start input");
        return;
    }
 
    let mut split_count = 0i64;

    for j in 1..diagram.len() {

        for i in 0..diagram[j].len() {
            let above = diagram[j - 1][i];
            if above == b'.' || above == b'^' {
                continue;
            }
    
            if diagram[j][i] == b'^' {
                if i > 0 && diagram[j][i - 1] != b'|' {
                        diagram[j][i - 1] = b'|';
                }
                
                if i < diagram[j].len() - 1 && diagram[j][i + 1] != b'|' {
                        diagram[j][i + 1] = b'|';
                }

                split_count = split_count + 1;
                continue;
            }

            diagram[j][i] = b'|';
        }
    }
    
    for row in &diagram {
        println!("{}", String::from_utf8_lossy(row));
    }
    
    println!("part one: {:?}", split_count);

    // playing with DP
    let mut memo: Vec<Vec<Option<i64>>> = vec![vec![None; w]; h];
    let mut total_paths = 0;
    let bottom_y = h - 1;

    for i in 0..w {
        let count = get_parent_count(i, bottom_y, &diagram, &mut memo);
        println!("column {}/{} => {}", i, w, count);
        total_paths = total_paths + count;

    }

    println!("second part: {:?}", total_paths);
}

fn get_parent_count(
    i: usize,
    j: usize,
    diagram: &Vec<Vec<u8>>,
    memo: &mut [Vec<Option<i64>>]
) -> i64 {
    if let Some(v) = memo[j][i] {
        return v;
    }

    let cell = diagram[j][i];

    if cell == b'.' {
        memo[j][i] = Some(0);
        return 0;
    }

    if j == 0 {
        memo[j][i] = Some(if cell == b'S' { 1 } else { 0 });
        return memo[j][i].unwrap();
    }

    if diagram[j - 1][i] == b'S' {
        memo[j][i] = Some(1);
        return 1;
    }

    let w = diagram[0].len();
    let mut total = 0i64;

    //from above
    total = total + get_parent_count(i, j - 1, diagram, memo);

    //from left and right
    if i > 0 && diagram[j][i - 1] == b'^' {
        total = total + get_parent_count(i - 1, j, diagram, memo)
    }

    if i + 1 < w && diagram[j][i + 1] == b'^' {
        total = total + get_parent_count(i + 1, j, diagram, memo)
    }

    memo[j][i] = Some(total);
    total

   
}