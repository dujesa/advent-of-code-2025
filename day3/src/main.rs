use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;

    for line in lines {
        let mut first_max = (0, 0);
        let mut second_max = (0, 0);

        let chars: Vec<char> = line.chars().collect();
        let chars_count = chars.len();

        first_max = find_max(&chars, 0, chars_count - 1);
        second_max = find_max(&chars, first_max.1 + 1, chars_count);

        let line_max = first_max.0 * 10 + second_max.0;
        println!("{}", line_max);
        result = result + line_max;
    }
    
    println!("part one {}", result); 
}

fn find_max (chars: &Vec<char>, from: i32, to: usize) -> (i32, i32) {
    let number  = &chars[(from as usize)..];
    
    let mut max: i32 = 0;
    let mut max_index: i32 = 0;
    let mut i: i32 = 0;

    for c in number {
        let check_num: i32 = c.to_digit(10).unwrap() as i32;
        if check_num > max {
            max = check_num;
            max_index = i;
        }

        i = i + 1;

        if i as usize >= to {
            break;
        }
    }

    println!("{} {}", max, max_index);
    (max, max_index)
}
