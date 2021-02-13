mod read_file;
use read_file::get_input;

fn get_lines(input: &str) -> Vec<&str> {
    input.split("\r\n").collect()
}

fn get_2d_map(lines: Vec<&str>) -> Vec<Vec<char>> {
    let per_line_len: f32 = lines[0].len() as f32;
    let rows_len = lines.len() as f32;
    let repeat_for: i32 = (rows_len / (per_line_len / 3.0).floor()).ceil() as i32 * 3;
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

fn count_trees(repeated_lines: &Vec<Vec<char>>, x_incrementer: usize, y_incrementer: usize ) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut trees: i64 = 0;
    for _ in 1..repeated_lines.len() {
        x += x_incrementer;
        y += y_incrementer;
        let map_y = match repeated_lines.get(y) {
            Some(c) => c,
            None => break
        };
        if map_y[x] == '#'{
            trees += 1;
        };
    };
    trees
}

fn count_and_multiply(repeated_lines: &Vec<Vec<char>>) -> i64 {
    let first_slope = count_trees(repeated_lines, 1, 1);
    let second_slope = count_trees(repeated_lines, 3, 1);
    let third_slope = count_trees(repeated_lines, 5, 1);
    let forth_slope = count_trees(repeated_lines, 7, 1);
    let fifth_slope = count_trees(repeated_lines, 1, 2);
    first_slope * second_slope * third_slope * forth_slope * fifth_slope
}


fn main() {
    let input = get_input("inputs/input_d3.txt");
    let map = get_2d_map(get_lines(&input));
    let answer = count_and_multiply(&map);
    println!("{}", answer);
}