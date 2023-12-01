use std::env;
use std::fs;
use std::fmt::Write;

fn line_value(input: &str) -> u32 {

    let first_index = input.find(char::is_numeric).unwrap();
    let last_index = input.rfind(char::is_numeric).unwrap();

    let first_digit = input.chars().nth(first_index).unwrap().to_digit(10).unwrap();
    let last_digit = input.chars().nth(last_index).unwrap().to_digit(10).unwrap();

    let sb = String::from("Hello World! {first_index} {last_index}");

    let var_name = (first_digit * 10) + last_digit;
    return var_name;
}


pub fn day1() {
    println!("Day1");

    let args: Vec<String> = env::args().collect();

    // input file name
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let lines = contents.split('\n').map(|x| line_value(x));

    let sum_of_lines: u32 = lines.clone().sum();

    let mut output = String::new();

    let lines_two = lines.clone();
    for value in lines_two {
        if output.len() > 0 {
            output.push_str(", ");
        }
        // ! discard/ignore any write error
        _ = write!(output, "{value}");
    }


    // let f = |acc, e| -> format!("{acc}\n{e}");
    // let line_output = lines.reduce(f).unwrap();

    println!("With values:\n{output}");

    
    println!("Total:\n{sum_of_lines}");
}