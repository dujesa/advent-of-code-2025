use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let matrix:Vec<Vec<&str>> = input.lines().map(|l| l.split_whitespace().collect()).into_iter().rev().collect();

    let mut result = 0;

    for j in 0..matrix[0].len() {
        let operator = matrix[0][j];
        let mut column_result = match operator {
            "*" => 1,
            _ => 0,
        };

        for i in 1..matrix.len() {
            let cell = matrix[i][j].parse::<i64>().expect("Invalid number");

            column_result = match operator {
                "+" => column_result + cell,
                "*" => column_result * cell,
                _ => column_result
            }
        }
        result = result + column_result;
    }

    println!("part one - {}", result);

    let input = fs::read_to_string("input.txt").unwrap();
    let grid:Vec<Vec<char>> = input.lines()
        .map(|l| l.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut split_cols = Vec::new();

    for col in 0..cols {
        if grid.iter().all(|row| row[col] == ' ') {
            split_cols.push(col);
        }
    }

    let mut split_col_ranges: Vec<(usize, usize)> = vec![(0, split_cols[0])];
    for i in 0..(split_cols.len() - 1) {
        split_col_ranges.push((split_cols[i] + 1, split_cols[i + 1]));
    }

    let mut matrix:Vec<Vec<Vec<char>>> = vec![vec![]; rows];
    let mut i = 0;

    for line in grid {
        let mut j = 0;
        for split_col_range in &split_col_ranges {
            matrix[i].push(line[split_col_range.0..split_col_range.1].to_vec());
            j = j + 1;
        }
        matrix[i].push(line[(split_col_ranges.last().unwrap().1 + 1)..].to_vec());

        i = i + 1;
    }        

    matrix.reverse();

    let mut result = 0;

    for j in 0..matrix[0].len() {
        let operator = matrix[0][j][0];
        let mut column_result = match operator {
            '*' => 1,
            _ => 0,
        };

        let mut column_numbers: Vec<Vec<char>> = Vec::new();
        for i in 1..matrix.len() {
             column_numbers.push(matrix[i][j].clone());
        }

        let celalophod_numbers = parse_cephalopod_numbers(&column_numbers);

        for celalophod_number in celalophod_numbers {

            column_result = match operator {
                '+' => column_result + celalophod_number,
                '*' => column_result * celalophod_number,
                _ => column_result
            }
        }
        result = result + column_result;
    }

    println!("part two - {}", result);
}

fn parse_cephalopod_numbers(matrix: &Vec<Vec<char>>) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();
    let max_digits_count = matrix.len();
    
    for j in 0..matrix[0].len() {
        let mut number: String = String::new();
        
        for i in 0..max_digits_count {
            if matrix[max_digits_count - i - 1].len() < j {
                number.push(' ');
                continue;
            }
            
            number.push(matrix[max_digits_count - i - 1][j]);
        }

        numbers.push(number.trim().parse().expect("invalid parsed number"))
    }

    numbers
}
