use std::env;
use std::fmt::Write;
use std::fs;

fn line_value_part1(input: &str) -> u32 {
    let first_index = input.find(char::is_numeric).unwrap();
    let last_index = input.rfind(char::is_numeric).unwrap();

    let first_digit = input
        .chars()
        .nth(first_index)
        .unwrap()
        .to_digit(10)
        .unwrap();
    let last_digit = input.chars().nth(last_index).unwrap().to_digit(10).unwrap();

    let result = (first_digit * 10) + last_digit;
    return result;
}

fn sanitize_line(input: &str) -> String {
    return input
        .replace("zero", "0")
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");
}

// fn sanitize_lines(input: String) -> Map<Split<_, &str>, impl Fn(&str) -> String> {
//     let result = input
//         .split("\n")
//         .map(|x| sanitize_line_thingy(x.to_ascii_lowercase()));
//     return result;
// }

fn replacer(input: &str, found: (usize, String)) -> String {
    let result = match found.1.as_str() {
        "one" => input.replace("one", "1"),
        "two" => input.replace("two", "2"),
        "three" => input.replace("three", "3"),
        "four" => input.replace("four", "4"),
        "five" => input.replace("five", "5"),
        "six" => input.replace("six", "6"),
        "seven" => input.replace("seven", "7"),
        "eight" => input.replace("eight", "8"),
        "nine" => input.replace("nine", "9"),
        "zero" => input.replace("zero", "0"),
        _ => String::from(input),
    };

    return result;
}

// fn gather_numbers(input: &str) -> Vec<u32> {
//     for i in (0..(input.len())) {
//         input.chars().nth(i).unwrap().to_digit(10).or_else(|| {
//             match
//         })
//     }
// }

fn lowest_index_of(input: &str, values: [&str; 18]) -> Option<(usize, String)> {
    let mut found_index: Option<usize> = None;
    let mut found_value: Option<String> = None;
    for value in values {
        let index = input.find(value);
        match index {
            Some(current_index) => {
                if found_index.is_none() || current_index <= found_index.unwrap_or(1000) {
                    found_index = Some(current_index);
                    found_value = Some(value.to_ascii_lowercase());
                }
            }
            None => {}
        }
    }

    // match found_value {
    //     Some(_) => {
    //         // replace value
    //         return input.replace(found_value)
    //     }
    //     None => {}
    // }
    let result = match found_value {
        Some(fv) => Some((found_index.unwrap(), fv)),
        None => None,
    };

    return result;
}

fn highest_index_of(input: &str, values: [&str; 18]) -> Option<(usize, String)> {
    let mut found_index: Option<usize> = None;
    let mut found_value: Option<String> = None;
    for value in values {
        let index = input.rfind(value);
        match index {
            Some(current_index) => {
                if found_index.is_none() || current_index >= found_index.unwrap_or(0) {
                    found_index = Some(current_index);
                    found_value = Some(value.to_ascii_lowercase());
                }
            }
            None => {}
        }
    }

    // match found_value {
    //     Some(_) => {
    //         // replace value
    //         return input.replace(found_value)
    //     }
    //     None => {}
    // }
    // return (found_index, found_value);
    let result = match found_value {
        Some(fv) => Some((found_index.unwrap(), fv)),
        None => None,
    };

    return result;
}

fn sanitize_line_thingy(input: &str) -> String {
    // bug: should replace from left to right, not just zero->nine

    // find first of any of the numbers and replace with the digit.
    // let mut mapped = HashMap::new();

    let str_values = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    // for value in str_values {
    //     mapped.insert(
    //         value.to_owned(),
    //         (
    //             input.find(value).unwrap_or(1000),
    //             input.rfind(value).unwrap_or(1000),
    //         ),
    //     );
    // }

    let low_input = input.clone();
    let high_input = input.clone();

    let lowest = lowest_index_of(low_input, str_values);
    let highest = highest_index_of(high_input, str_values);

    // find the smallest and largest values (not 1000)

    // let mut smallest: usize = 1000;
    // let mut smallest_str: String = String::new();
    // let mut largest: usize = 0;
    // let mut largest_str: String = String::new();

    // for hv in mapped {
    //     // firsts
    //     if hv.1 .0 < smallest {
    //         smallest = hv.1 .0;
    //         smallest_str = hv.0.clone();
    //     }
    //     if hv.1 .1 > largest {
    //         largest = hv.1 .1;
    //         largest_str = hv.0.clone();
    //     }
    // }

    let input1 = match lowest {
        Some(low_v) => replacer(input, low_v),
        None => String::from(input),
    };

    let input2 = match highest {
        Some(high_v) => replacer(input1.as_str(), high_v),
        None => String::from(input1),
    };

    println!("day2::x::sanitize {input2}");
    return String::from(input2);
}

fn line_value_part2(input: String) -> u32 {
    // let input_sanitized = sanitize_line(input);

    let first_index = input.find(char::is_numeric).unwrap();
    let last_index = input.rfind(char::is_numeric).unwrap();

    let first_digit = input
        .chars()
        .nth(first_index)
        .unwrap()
        .to_digit(10)
        .unwrap();
    let last_digit = input.chars().nth(last_index).unwrap().to_digit(10).unwrap();

    // println!("p2,l,from:{input},to:{input_sanitized},fd:{first_digit},ld:{last_digit}\n");

    let result = (first_digit * 10) + last_digit;
    return result;
}

fn part1(contents: String) {
    let lines = contents.split('\n').map(|x| line_value_part1(x));

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

    // println!("With values:\n{output}");

    println!("Part 1 Total:\n{sum_of_lines}");
}

fn part2(contents: String) {
    // let lines_x = contents.split('\n').map(|x| update_line(x));

    // let sanitized_lines = sanitize_lines(contents);
    let sanitized_lines = contents.split("\n").map(|x| sanitize_line_thingy(x));

    let lines = sanitized_lines.map(|x| line_value_part2(x));

    let sum_of_lines: u32 = lines.clone().sum();

    // let mut output = String::new();

    let lines_two = lines.clone();
    for value in lines_two {
        // if output.len() > 0 {
        //     output.push_str(", ");
        // }
        // // ! discard/ignore any write error
        // _ = write!(output, "{value}");
        println!("With values: {value}");
    }

    // let f = |acc, e| -> format!("{acc}\n{e}");
    // let line_output = lines.reduce(f).unwrap();

    // println!("With values:\n{output}");

    println!("Part 2 Total:\n{sum_of_lines}");

    // debug
    let str_values = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let input = "eight26vhjjz4foureightwojk";

    let highest_item = highest_index_of(input, str_values);
    match highest_item {
        Some(value) => {
            let x = value.0;
            let y = value.1;
            println!("Yes {x} {y}");
            let r = replacer(input, (x, y));
            println!("Replaced {input} {r}");
        }
        None => {
            println!("No")
        }
    }

    // let input2 = match highest_item {
    //     Some(hv) => {
    //         return replacer(input, hv.1)
    //     },
    //     None => String::from(input),
    // };
    // let input2 = match highest_item {
    //     Some(high_v) => replacer(input, high_v.1),
    //     None => String::from(input),
    // };

    // println!("Replacer: {input} -> {input2}")
}

pub fn day1() {
    println!("Day1");

    let args: Vec<String> = env::args().collect();

    // input file name
    let file_path = &args[2];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let contents1 = contents.clone();
    part1(contents1);

    let contents2 = contents.clone();
    part2(contents2);

    // should be 53348 but see 53373
    let contents3 = contents.clone();
    let result = json_part2(contents3);

    println!("In file {}", result);
}

fn json_part2(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars_vec = line.chars().collect::<Vec<_>>();
            let digits = (0..chars_vec.len())
                .filter_map(|i| {
                    chars_vec[i].to_digit(10).or_else(|| {
                        let remainder: String = chars_vec.iter().skip(i).collect();
                        if remainder.starts_with("one") {
                            Some(1)
                        } else if remainder.starts_with("two") {
                            Some(2)
                        } else if remainder.starts_with("three") {
                            Some(3)
                        } else if remainder.starts_with("four") {
                            Some(4)
                        } else if remainder.starts_with("five") {
                            Some(5)
                        } else if remainder.starts_with("six") {
                            Some(6)
                        } else if remainder.starts_with("seven") {
                            Some(7)
                        } else if remainder.starts_with("eight") {
                            Some(8)
                        } else if remainder.starts_with("nine") {
                            Some(9)
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>();
            let first = digits[0];
            let last = *digits.last().unwrap();

            // println!("JSON {first}{last}");
            first * 10 + last
        })
        .sum()
}
