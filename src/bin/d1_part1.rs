mod read_file;
use read_file::get_input;


fn parse_input(input: &str) -> Vec<i32> {
    let strs: Vec<&str> = input.split("\r\n").collect();
    let mut new_vec: Vec<i32> = Vec::new();
    for str_ in strs {
        match str_.parse::<i32>() {
            Ok(num) => new_vec.push(num),
            Err(bruh) => println!("bruh err: {:?}", bruh)
        };
    };
    new_vec
}

fn find_total_2020(nums: Vec<i32>) -> Option<(i32, i32)> {
    for num_base in &nums {
        for num in &nums {
            if num_base == num {
                continue;
            };
            if num_base + num == 2020 {
                return Some((*num_base, *num));
            };
        };
    };
    None
}

fn mult_and_get_the_answer(num_tple: &(i32, i32)) -> i32{
    num_tple.0 * num_tple.1
}

fn main() {
    let input = get_input("inputs/input_d1.txt");
    let parsed_input = parse_input(&input);
    let found_2020s = find_total_2020(parsed_input).unwrap();
    let answer = mult_and_get_the_answer(&found_2020s);
    println!("{}", answer);
}
