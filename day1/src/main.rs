use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let rotations: Vec<(char, i16)> = input
        .lines()
        .map(|s| {
            let direction = s.chars().next().unwrap();
            let number = s[1..].parse().unwrap();

            (direction, number)
        })
        .collect();

    let mut dial: i16 = 50;
    let mut zero_hit_count = 0;
    let mut zero_pass_count = 0;
    
    for (direction, number) in rotations {
        println!("{direction} {number}");
        
        zero_pass_count = zero_pass_count + number / 100;
        let number = number % 100;
        let initial_dial = dial;

        dial = if direction == 'R' {
            dial + number
        } else {
            dial - number
        };
        
        dial = if dial > 0 {
            dial
        } else if dial == 0 {
            zero_pass_count = zero_pass_count + 1; 
            dial
        } else {
            if initial_dial != 0 {
                zero_pass_count = zero_pass_count + 1;
            }
            100 + dial
        };
        
        dial = if dial > 99 {
            zero_pass_count = zero_pass_count + 1;
            dial % 100
        } else {
            dial
        };
        println!("Current dial: {dial}");

        zero_hit_count = if dial == 0 {
            zero_hit_count + 1
        } else {
            zero_hit_count
        };
    }
    
    println!("Password part 1 is: {zero_hit_count}");
    println!("Password part 2 is: {zero_pass_count}");
}
