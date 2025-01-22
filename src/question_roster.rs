
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::question::{Question, AtomicQuestion, SequenceQuestion};

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

    pub fn get_bottom_level_limit(&self) -> usize {
        let min_score = self.questions.first().unwrap().min_score();

        self.questions
            .iter()
            .take_while(|q| q.min_score() == min_score)
            .count()
    }
}
