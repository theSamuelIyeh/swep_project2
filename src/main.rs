use std::io;
use std::io::Write;
use colored::*;

fn main() {
    println!("\n{}", "Welcome to the Baby Weight Management System!".bright_green().bold());
    println!("\n{}", "Usable commands:".bright_magenta().bold().underline());
    println!("{}", "new <weight>: To add new baby weight.".bright_yellow().bold());
    println!("{}", "min: to get overall minimum baby weight.".bright_yellow().bold());
    println!("{}", "max: to get overall maximum baby weight.".bright_yellow().bold());
    println!("{}", "mean: to get average baby weight.".bright_yellow().bold());
    println!("{}", "quit: to quit the system.".bright_red().bold());
    let mut weights: Vec<f32> = Vec::new();
    loop {
        println!("\n{}", "******************************".bright_yellow().bold());
        print!("{}", "$>".bright_green().bold());
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Unable to read input");
        match command.as_str() {
            "quit\n" => {
                println!("{}\n", "Thank you, Good Bye!".bright_blue().bold());
                break
            },
            _ => {check_command(command, &mut weights)},
        }
    }
}

fn check_command(command: String, weights: &mut Vec<f32>) {
    let main_command: &str = command.split(&[' ', '\n']).nth(0).expect("error splitting command");
    match main_command {
        "new" => add_new_weight(command, weights),
        "min" => calc_min(weights),
        "max" => calc_max(weights),
        "mean" => calc_mean(weights),
        _ => println!("{}", "Invalid Command!!".red().bold()),
    }
}

fn add_new_weight(command: String, weights: &mut Vec<f32>) {
    let weight = command.split(" ").nth(1).expect("Error splitting weight");
    match weight.trim().parse() {
        Ok(value) => {
            weights.push(value);
            println!("{} {}", value.to_string().bright_magenta(), "Added!".bright_magenta().bold());
        },
        Err(_) => println!("{}", "Enter a valid weight".red().bold()),
    };
}

fn calc_min(weights: &mut Vec<f32>) {
    let mut min: f32 = weights[0];
    for weight in weights {
        if weight < &mut min {
            min = *weight;
        }
    }
    println!("{} {}", "MIN is :".bright_green().bold(), min.to_string().bright_yellow().bold());
}

fn calc_max(weights: &mut Vec<f32>) {
    let mut max: f32 = weights[0];
    for weight in weights {
        if weight > &mut max {
            max = *weight;
        }
    }
    println!("{} {}", "MAX is :".bright_green().bold(), max.to_string().bright_yellow().bold());
}

fn calc_mean(weights: &mut Vec<f32>) {
    let mut sum: f32 = 0.0;
    let len: f32 = weights.len() as f32;
    for weight in weights {
        sum += *weight;
    }
    let average = sum / len;
    println!("{} {}", "Average is :".bright_green().bold(), average.to_string().bright_yellow().bold());
}
