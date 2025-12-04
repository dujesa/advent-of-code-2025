use std::fs;

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn main() {
    let paper_grid_input: Vec<Vec<char>> = fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|row| row.chars().collect())
        .collect();
    const MAX_NEIGHOURS: i32 = 4;

    let mut paper_grid = paper_grid_input.clone();

    let mut total_accessable = 0;

    let max_y = paper_grid.len() as i32; 
    let max_x = paper_grid[0].len() as i32;
    let mut y = 0;
    let mut x = 0;
    
    while y < max_y {
        while x < max_x {
            let neighbours = get_neighbours(Coordinate { x, y }, &paper_grid_input);

            let paper_neighbours_count = paper_neighbours_count(&mut paper_grid, &neighbours);
            let is_accessable = MAX_NEIGHOURS > paper_neighbours_count;

            if is_accessable == true && paper_grid[y as usize][x as usize] == '@'{ 
                paper_grid[y as usize][x as usize] = 'x';
                total_accessable = total_accessable + 1 
            };
            
            print!("{paper_neighbours_count}");

            x = x + 1;
        }
        println!();
        y = y + 1;
        x = 0;
    }

    print_grid(&paper_grid_input);
    print_grid(&paper_grid);

    println!("part one - {}", total_accessable);

    let mut paper_grid = paper_grid_input.clone();

    let max_y = paper_grid.len() as i32; 
    let max_x = paper_grid[0].len() as i32;
    let mut y = 0;
    let mut x = 0;

    let mut round_accessable = total_accessable;
    total_accessable = 0;
    
    while round_accessable > 0 {
        round_accessable = 0;
        while y < max_y {
            while x < max_x {
                let neighbours = get_neighbours(Coordinate { x, y }, &paper_grid_input); 

                let paper_neighbours_count = paper_neighbours_count(&mut paper_grid, &neighbours);
                let is_accessable = MAX_NEIGHOURS > paper_neighbours_count;
                
                if is_accessable == true && paper_grid[y as usize][x as usize] == '@'{ 
                    paper_grid[y as usize][x as usize] = 'x';
                    round_accessable = round_accessable + 1;
                };
                
                x = x + 1;
            }
            println!();
        
            y = y + 1;
            x = 0;
        }

        x = 0;
        y = 0;

        remove_xes(&mut paper_grid);
        print_grid(&paper_grid);
        total_accessable = total_accessable + round_accessable;
        
    } 

    println!("part 2 - {total_accessable}");
}

fn get_neighbours(roll: Coordinate, grid: &Vec<Vec<char>>) -> Vec<Coordinate> {
    let max_y = (grid.len() - 1) as i32; 
    let max_x = (grid[0].len() - 1) as i32;

    let x_candidates  = vec![roll.x - 1, roll.x, roll.x + 1];
    let y_candidates = vec![roll.y - 1, roll.y, roll.y + 1];

    let mut neighbours: Vec<Coordinate> = Vec::new();

    for &y in &y_candidates {
        if y < 0 || y > max_y { continue; }

        for &x in &x_candidates {
            if x < 0 || x > max_x { continue; }
            if x == roll.x && y == roll.y { continue; }

            neighbours.push(Coordinate { x, y });
        }
    }

    neighbours
}

fn paper_neighbours_count (grid: &mut Vec<Vec<char>>, neighbours: &Vec<Coordinate>) -> i32 {
    let mut neighbour_roll_count = 0;

    for neighbour in neighbours {
        if grid[neighbour.y as usize][neighbour.x as usize] != '.' {
            neighbour_roll_count = neighbour_roll_count + 1;
        }
    }
    
    neighbour_roll_count
}

fn remove_xes (grid: &mut Vec<Vec<char>>) {
    let max_y = grid.len() as usize; 
    let max_x = grid[0].len() as usize;
    let mut y = 0;
    let mut x = 0;
    
    while y < max_y {
        while x < max_x {
            if grid[y][x] == 'x' {
                grid[y][x] = '.';
            }

            x = x + 1;
        }
        y = y + 1;
        x = 0
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    let max_y = grid.len() as usize; 
    let max_x = grid[0].len() as usize;
    let mut y = 0;
    let mut x = 0;

    while y < max_y {
        while x < max_x {
            print!("{}", grid[y][x]);

            x = x + 1;
        }
        println!();
        y = y + 1;
        x = 0;
    }
    println!();
}