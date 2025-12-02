use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let id_ranges = input.split(',').enumerate();

    let mut result: i64 = 0;

    for (_i,id_range) in id_ranges {
        let min_max: Vec<i128> = id_range.split('-').map(|id| id.parse::<i128>().unwrap()).collect();
        
        let min = min_max[0];
        let max = min_max[1];

        let mut check_num = min;

        while check_num <= max {
            let check_string = check_num.to_string();
            check_num = check_num + 1;
            
            if check_string.len() % 2 == 1 {
                continue;
            }

            if check_string[..check_string.len()/2] != check_string[check_string.len()/2..] {
                continue;
            }

            result = result + check_string.parse::<i64>().unwrap();
        }
    }

    print!("part one = {}", result);

    let input = fs::read_to_string("input.txt").unwrap();
    let id_ranges = input.split(',').enumerate();

    let mut result: i64 = 0;

    for (_i,id_range) in id_ranges {
        let min_max: Vec<i128> = id_range.split('-').map(|id| id.parse::<i128>().unwrap()).collect();
        
        let min = min_max[0];
        let max = min_max[1];

        let mut check_num = min;

        'outer: while check_num <= max {
            let check_string = check_num.to_string();
            check_num = check_num + 1;

            let divisors = provide_divisors(check_string.len().try_into().unwrap());
            if divisors.len() == 0 {
                continue;
            }

            for divisor in divisors {
                let substrings = split_strings(&check_string, divisor as usize);
                let check_substring = substrings.first().unwrap();

                let all_same = substrings.iter().all(|s| s == check_substring);
            
                if !all_same {
                    continue;
                }

                result = result + check_string.parse::<i64>().unwrap();
                continue 'outer;
            }

        }
    }

    println!("part two = {}", result);
}

fn provide_divisors (num: i128) -> Vec<i128> {
    let mut check_num = 1;
    let mut divisors: Vec<i128> = Vec::new();

    while check_num < num {
        if num % check_num == 0 {
            divisors.push(check_num);
        }
        check_num = check_num + 1;
    }

    divisors
}

fn split_strings (string: &str, chunk_size: usize) -> Vec<String> {
    string.chars().collect::<Vec<char>>().chunks(chunk_size)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

