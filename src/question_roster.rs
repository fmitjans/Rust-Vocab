
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::question::{Question};

pub struct QuestionRoster {
    pub questions: Vec<Question>,
}

impl QuestionRoster {

    pub fn new(questions: Vec<Question>) -> Self {
        Self {
            questions,
        }
    }

    pub fn shuffle_by_scores(&mut self) {
        
        self.questions.shuffle(&mut thread_rng());

        self.questions.sort_by(|a, b| a.min_score().cmp(&b.min_score()));
    }

    pub fn interrogate_lowest(&mut self) {
        let bottom_level_limit = self.get_bottom_level_limit();

        for i in 0..bottom_level_limit {
            self.questions[i].interrogate();
        }
    }

    fn get_bottom_level_limit(&self) -> usize {
        let min_score = self.questions.first().unwrap().min_score();

        let question_count = self.questions
            .iter()
            .take_while(|q| q.min_score() == min_score)
            .count();
        
        // println!("Asking {} questions with score {}", question_count, min_score);

        question_count
    }
}
