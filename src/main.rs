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
    let mut zero_count = 0;
    
    for (direction, number) in rotations {
        println!("{direction} {number}");
        
        dial = if direction == 'R' {
            dial + number
        } else {
            dial - number
        };
        
        dial = if dial > 0 {
            dial
        } else {
            100 + dial
        };
        
        dial = dial % 100;
        println!("Current dial: {dial}");

        zero_count = if dial == 0 {
            zero_count + 1
        } else {
            zero_count
        };
    }
    
    println!("Password is: {zero_count}");
}
