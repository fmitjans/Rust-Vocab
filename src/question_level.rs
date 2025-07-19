use crate::question::{Question, ScoreType};
use crate::question_roster::{QuestionRoster};

#[derive(Clone)]
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

    pub fn interrogate(&mut self, roster_ref: &mut QuestionRoster) {
        for question in &mut self.questions {
            question.interrogate(roster_ref);
        }
    }

    pub fn decrease_score(&mut self, amount: ScoreType) {
        self.score -= amount;
        for question in &mut self.questions {
            question.decrease_score(amount);
        }
    }
}