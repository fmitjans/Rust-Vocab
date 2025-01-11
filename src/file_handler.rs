use std::path::Path;
use std::fs;
use std::io;
use std::ops::RangeInclusive;

use super::params::{QUESTIONS_FOLDER};

pub struct FileReader;

impl FileReader {
    pub fn load_questions() {

        let mut available_files = get_file_names();

        available_files.sort();
        
        print_options(&available_files);

        let valid_range = RangeInclusive::new(1, available_files.len());
        let selected_numbers = get_valid_user_numbers(&valid_range);
    }
}

fn get_file_names() -> Vec<String> {
    let path = Path::new(QUESTIONS_FOLDER);

    fs::read_dir(path)
        .expect("Error reading questions folder")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter_map(|path| path.file_name()?.to_str().map(String::from))
        .collect()
}

fn print_options(file_names: &Vec<String>) {

    for (index, name) in file_names.iter().enumerate() {
        println!("{}. {}", index + 1, name);
    }
}

fn get_valid_user_numbers(valid_range: &RangeInclusive<usize>) -> Vec<usize> {

    loop {
        let numbers = ask_for_numbers();

        match validate_numbers(&numbers, valid_range) {

            Ok(()) => return numbers,

            Err(invalid_numbers) => {
                println!(
                    "Numbers {:?} are out of range. Please choose numbers between {} and {}",
                    invalid_numbers,
                    valid_range.start(),
                    valid_range.end()
                );
            }
        }
    }
}

fn ask_for_numbers() -> Vec<usize> {

    println!("Select file numbers:");

    let user_input = read_input();
    to_number_vec(&user_input)
}

fn read_input() -> String {

    let mut numbers_string = String::new();
    io::stdin()
        .read_line(&mut numbers_string)
        .unwrap();

    numbers_string
}

fn to_number_vec(numbers_string: &str) -> Vec<usize> {
    numbers_string
    .trim()
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect()
}

fn validate_numbers(numbers: &[usize], valid_range: &RangeInclusive<usize>) -> Result<(), Vec<usize>> {
    let invalid_numbers: Vec<usize> = numbers
        .iter()
        .filter(|&number| !valid_range.contains(number))
        .copied()
        .collect();

    if invalid_numbers.is_empty() {
        Ok(())
    } else {
        Err(invalid_numbers)
    }
}