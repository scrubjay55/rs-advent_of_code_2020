mod read_file;
use read_file::get_input;

fn get_lines(input: &str) -> Vec<&str> {
    input.split("\r\n").collect()
}

fn get_2d_map(lines: Vec<&str>) -> Vec<Vec<char>> {
    let per_line_len: f32 = lines[0].len() as f32;
    let rows_len = lines.len() as f32;
    let repeat_for: i32 = (rows_len / (per_line_len / 3.0).floor()).ceil() as i32;
    let mut new_lines: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut repeated_lines = String::new();
        for _ in 0..repeat_for {
            repeated_lines.push_str(line.trim());
        };
        new_lines.push(repeated_lines.chars().collect());
    };
    new_lines
}

fn count_trees(repeated_lines: &Vec<Vec<char>>) -> i32 {
    let mut x = 3;
    let mut y = 1;
    let mut trees = 0;
    for _ in 1..repeated_lines.len() {
        if repeated_lines[y][x] == '#'{
            trees += 1;
        };
        x += 3;
        y += 1;
    };
    trees
}

fn main() {
    let input = get_input("inputs/input_d3.txt");
    let map = get_2d_map(get_lines(&input));
    let trees_count = count_trees(&map);
    println!("{}", trees_count);
}