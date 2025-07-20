use crate::question::{Question, ScoreType};

#[derive(Debug, Clone)]
pub struct QuestionLevel {
    pub questions: Vec<Question>,
    pub length: usize,
    pub score: ScoreType,
}

impl QuestionLevel {
    pub fn new(questions: Vec<Question>) -> Self {
        let length = questions.len();
        let score = questions.first().unwrap().min_score();
        Self {
            questions,
            length,
            score,
        }
    }
}