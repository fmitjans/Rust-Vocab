use std::fs;
use serde::{Deserialize, Serialize};
//use serde_json::Result;

pub fn read_json(file_path: &str) -> Vec<Question> {
    assert!(file_path.ends_with(".json"), "File is not json");
    let data = fs::read_to_string(file_path).unwrap();
    let q_vec: Vec<Question> = serde_json::from_str(&data).unwrap();

    q_vec
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AtomicQuestion {
    question: String,
    answer: String,
    #[serde(default)]
    note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceQuestion {
    score: u32,
    content: Vec<AtomicQuestion>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleQuestion {
    score: u32,
    #[serde(flatten)]
    content: AtomicQuestion,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Question {
    #[serde(rename = "sequence")]
    SequenceQuestion(SequenceQuestion),
    #[serde(rename = "simple")]
    SimpleQuestion(SimpleQuestion),
}