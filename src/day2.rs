use std::{env, fs};

fn parse_round_item(input: &str) -> (u32, u32, u32) {
    // 3 blue, 4 red;
    // 1 red, 2 green, 6 blue;
    // 2 green

    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;

    let pieces = input.split(",").map(|line| {
        let parts: Vec<&str> = line.trim().split(" ").collect();
        let num = parts[0].parse::<u32>().unwrap_or(0);
        let color = parts[1];
        (num, color)
    });

    for piece in pieces {
        match piece.1 {
            "red" => r = piece.0,
            "green" => g = piece.0,
            "blue" => b = piece.0,
            _ => {}
        }
    }

    return (r, g, b);
}

fn parse_round(input: &str, max_balls: (u32, u32, u32)) -> Option<u32> {
    let mut round_values = input.split(",").map(|i| {
        let r_values = parse_round_item_part2(i);

        if r_values.0 <= max_balls.0 && r_values.1 <= max_balls.1 && r_values.2 <= max_balls.2 {
            Some(1)
        } else {
            None
        }
    });

    let all_valid_items = round_values.all(|v| v.is_some());

    if all_valid_items {
        Some(1)
    } else {
        None
    }
}

fn parse_game(game_id: u32, input: &str, max_balls: (u32, u32, u32)) -> Option<u32> {
    let rounds = input.split(";");
    let mut round_items = rounds.map(|r| parse_round(r, max_balls));
    let all_valid_rounds = round_items.all(|v| v.is_some());

    if all_valid_rounds {
        println!("Game {game_id} [{all_valid_rounds}] {input} -> {all_valid_rounds}");
    }
    if all_valid_rounds {
        Some(game_id)
    } else {
        None
    }
}

fn part1(content: String) -> u32 {
    let max_balls = (12, 13, 14);

    let result = content.lines().filter_map(|line| {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let s1: Vec<&str> = line.split(":").collect();
        let game_id = s1[0].replace("Game ", "").parse::<u32>().unwrap_or(0);
        // let rounds = s1[1].trim().split(";");
        // let mut round_items = rounds.map(|r| parse_round(r, max_balls));
        // let all_valid_rounds = round_items.all(|v| v.is_some());

        // println!("Game {game_id} {all_valid_rounds}");
        // if all_valid_rounds {
        //     Some(game_id)
        // } else {
        //     None
        // }
        parse_game(game_id, s1[1].trim(), max_balls)
    });

    return result.sum();
}

fn part2(content: String) -> u32 {
    let result = content.lines().filter_map(|line| {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let s1: Vec<&str> = line.split(":").collect();
        let game_id = s1[0].replace("Game ", "").parse::<u32>().unwrap_or(0);
        // let rounds = s1[1].trim().split(";");
        // let mut round_items = rounds.map(|r| parse_round(r, max_balls));
        // let all_valid_rounds = round_items.all(|v| v.is_some());

        // println!("Game {game_id} {all_valid_rounds}");
        // if all_valid_rounds {
        //     Some(game_id)
        // } else {
        //     None
        // }
        parse_game_part2(game_id, s1[1].trim())
    });

    return result.sum();
}

fn parse_round_item_part2(input: &str) -> (u32, u32, u32) {
    // 3 blue, 4 red;
    // 1 red, 2 green, 6 blue;
    // 2 green

    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;

    let pieces = input.split(",").map(|line| {
        let parts: Vec<&str> = line.trim().split(" ").collect();
        let num = parts[0].parse::<u32>().unwrap_or(0);
        let color = parts[1];
        (num, color)
    });

    for piece in pieces {
        match piece.1 {
            "red" => r = piece.0,
            "green" => g = piece.0,
            "blue" => b = piece.0,
            _ => {}
        }
    }

    return (r, g, b);
}

fn parse_round_part2(
    input: &str,
    mut max_required_balls: (u32, u32, u32),
) -> Option<(u32, u32, u32)> {
    // println!(
    //     "round {input}, using max {} {} {}",
    //     max_required_balls.0, max_required_balls.1, max_required_balls.2
    // );
    let round_values = input.split(",").map(|i| {
        let r_values = parse_round_item(i.trim());

        if r_values.0 > max_required_balls.0 {
            max_required_balls.0 = r_values.0.clone();
        }
        if r_values.1 > max_required_balls.1 {
            max_required_balls.1 = r_values.1.clone();
        }

        if r_values.2 > max_required_balls.2 {
            max_required_balls.2 = r_values.2.clone();
        }

        // println!(
        //     "max_r: {} {} {}, {} {} {} <- {}",
        //     r_values.0,
        //     r_values.1,
        //     r_values.2,
        //     max_required_balls.0,
        //     max_required_balls.1,
        //     max_required_balls.2,
        //     i.trim(),
        // );

        Some(max_required_balls)
    });

    let all_valid_items = round_values.last().unwrap().unwrap();

    Some(all_valid_items)
}

fn parse_game_part2(game_id: u32, input: &str) -> Option<u32> {
    let mut max_required_balls = (0, 0, 0);
    let rounds = input.split(";");

    for round in rounds {
        let result = parse_round_part2(round, max_required_balls);
        max_required_balls = result.unwrap();

        // println!(
        //     "res {} {} {} = {}",
        //     max_required_balls.0,
        //     max_required_balls.1,
        //     max_required_balls.2,
        //     max_required_balls.0 * max_required_balls.1 * max_required_balls.2
        // );
        // Some(max_required_balls.0 * max_required_balls.1 * max_required_balls.2)
    }
    // let mut round_items = rounds.map(|r| {

    // });
    // let all_valid_rounds = round_items.all(|v| v.is_some());

    // let sum_rounds = round_items.last().unwrap().unwrap_or(1);
    let sum_rounds = max_required_balls.0 * max_required_balls.1 * max_required_balls.2;

    //if all_valid_rounds {
    println!("Game {game_id} {input}: {sum_rounds}");
    //}
    //if all_valid_rounds {
    Some(sum_rounds)
    // } else {
    //     None
    // }
}

pub fn day2() {
    println!("Day 2");

    let args: Vec<String> = env::args().collect();

    // input file name
    let file_path = &args[2];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents1 = contents.clone();
    println!("With text:\n{contents1}");

    let result = part1(contents1);

    println!("Result {result}");

    let contents2 = contents.clone();
    let result2 = part2(contents2);

    println!("Result2 {result2}")
}
