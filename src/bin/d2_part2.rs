mod read_file;
use read_file::get_input;

fn get_lines(input: &str) -> Vec<&str> {
    input.split("\r\n").collect()
}

fn parse_lines(lines: Vec<&str>) -> Vec<((i32, i32), char, &str)> {
    let mut parsed_lines: Vec<((i32, i32), char, &str)> = Vec::new();
    for line in &lines {
        let line_s: Vec<&str> = line.split(' ').collect();

        let line_s_zero: Vec<&str> = line_s[0].split('-').collect();
        let max_min_tuple = (line_s_zero[0].parse::<i32>().unwrap(), line_s_zero[1].parse::<i32>().unwrap());
        
        let p_char: char = line_s[1].chars().next().unwrap();
        let password: &str = line_s[2];

        parsed_lines.push((max_min_tuple, p_char, password));
    };
    parsed_lines
}

fn check_if_valid(line: &((i32, i32), char, &str)) -> bool { 
    let pass = line.2;
    let required_char = line.1;
    let chars_of_pass: Vec<char> = pass.chars().collect();

    let letter_1: Option<&char> = chars_of_pass.get(line.0.0 as usize - 1);
    let letter_2: Option<&char> = chars_of_pass.get(line.0.1 as usize - 1);

    let is_l1: bool = letter_1.is_some() && &required_char == letter_1.unwrap();
    let is_l2: bool = letter_2.is_some() && &required_char == letter_2.unwrap();
    is_l1 != is_l2
}

fn check_lines(lines: &Vec<((i32, i32), char, &str)>) -> i32 {
    let mut correct_lines = 0;
    for line in lines {
        if check_if_valid(line) {correct_lines += 1};
    };
    correct_lines
}

fn main() {
    let input = get_input("inputs/input_d2.txt");
    let parsed_lines = parse_lines(get_lines(&input));
    let correct_passwords = check_lines(&parsed_lines);
    println!("{}", correct_passwords);
}