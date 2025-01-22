use std::path::Path;
use std::fs;
use std::io;
use std::ops::RangeInclusive;

use crate::params::{QUESTIONS_FOLDER};
use crate::question::{self, Question};


pub struct FileReader;

impl FileReader {
    pub fn load_questions() -> Vec<Question> {

        let mut available_files = get_file_names();

        available_files.sort();
        
        print_options(&available_files);

        let valid_range = RangeInclusive::new(1, available_files.len());
        let selected_number = get_valid_user_numbers(&valid_range)[0];
        let selected_file_name = &available_files[selected_number - 1];
        
        let selected_file_path = Path::new(QUESTIONS_FOLDER)
        .join(selected_file_name)
        .to_str()
        .unwrap()
        .to_string(); // Convert to owned String
    
        let mut question_vec = read_json(&selected_file_path);
        question_vec
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


fn read_json(file_path: &str) -> Vec<Question> {
    assert!(file_path.ends_with(".json"), "File is not json");
    let data = fs::read_to_string(file_path).unwrap();
    let mut q_vec: Vec<Question> = serde_json::from_str(&data).unwrap();

    q_vec
}