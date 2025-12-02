use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let id_ranges = input.split(',').enumerate();

    let mut result = 0;

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

    print!("{}", result);


}

