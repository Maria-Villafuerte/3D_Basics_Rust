use std::fs::File;
use std::io::{BufRead, BufReader};
use nalgebra_glm::Vec2;

pub fn load_maze(filename: &str) -> (Vec<Vec<char>>, Vec2) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut finish_line = Vec2::new(0.0, 0.0);

    for (j, line) in reader.lines().enumerate() {
        let row: Vec<char> = line.unwrap().chars().collect();
        for (i, &cell) in row.iter().enumerate() {
            if cell == 'g' {
                finish_line = Vec2::new(i as f32 * 100.0 + 50.0, j as f32 * 100.0 + 50.0);
            }
        }
        maze.push(row);
    }

     // If no 'g' was found, set a default finish line
     if finish_line == Vec2::new(0.0, 0.0) {
        finish_line = Vec2::new(1300.0, 1300.0);  // Adjust this based on your maze size
    }

    (maze, finish_line)
}

pub fn is_wall(maze: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let i = x / 100;
    let j = y / 100;
    if j >= maze.len() || i >= maze[j].len() {
        println!("Out of bounds: Trying to access [{}, {}] in maze", j, i);
        return true;
    }
    let cell = maze[j][i];
    cell == '+' || cell == '-' || cell == '|'  // Note: 'g' is not considered a wall anymore
}