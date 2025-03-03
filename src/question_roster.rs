
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::question::{Question};
use crate::file_handler::save_json;

#[derive(Clone)]
pub struct QuestionRoster {
    pub questions: Vec<Question>,
    pub current_question_index: usize,
    pub bottom_limit_index: usize,
    pub is_ordered_ascending: bool,
}

pub enum Order {
    Ascending,
    Descending,
}

impl QuestionRoster {

    pub fn new(questions: Vec<Question>) -> Self {
        Self {
            questions: questions,
            current_question_index: 0,
            bottom_limit_index: 0,
            is_ordered_ascending: false,
        }
    }

    pub fn shuffle_questions(&mut self) {
        self.questions.shuffle(&mut thread_rng());
        self.is_ordered_ascending = false;
    }

    pub fn sort_by_scores(&mut self, order: Order) {
        // self.questions.sort_by(|a, b| a.min_score().cmp(&b.min_score()));
        match order {
            Order::Ascending => {
                self.questions.sort_by(|a, b| a.min_score().cmp(&b.min_score()));
                self.is_ordered_ascending = true;
            },
            Order::Descending => {
                self.questions.sort_by(|a, b| b.min_score().cmp(&a.min_score()));
                self.is_ordered_ascending = false;
            },
        }
    }

    pub fn interrogate_lowest(&mut self) {

        self.ensure_ordered();

        self.set_bottom_level_limit();

        println!("Commands: 1. Save 2. Skip Neutral 3. Skip Correct 4. Toggle Skip\n");

        while self.current_question_index < self.bottom_limit_index {

            let mut current_question = self.questions[self.current_question_index].clone();

            current_question.interrogate(self);

            self.questions[self.current_question_index] = current_question;

            self.current_question_index += 1;
        }
    }

    pub fn even_out_scores(&mut self) {

        self.ensure_ordered();

        let lowest_score = self.questions.first().unwrap().min_score();
        for question in self.questions.iter_mut() {
            question.decrease_score(lowest_score);
        }

    }

    pub fn save(&mut self, file_name: &str) {
        self.even_out_scores();
        self.print_levels();
        self.sort_by_scores(Order::Descending);
        save_json(&self.questions, file_name);
    }

    pub fn print_levels(&mut self) {

        self.ensure_ordered();

        let mut total_count = 0;
        let mut level_count = 0;
        let mut current_level_score = self.questions.first().unwrap().min_score();
        
        for question in self.questions.iter() {
            if question.min_score() == current_level_score {
                level_count += 1;
            } else {
                println!("{} questions with score {}", level_count, current_level_score);
                total_count += level_count;
                level_count = 1;
                current_level_score = question.min_score();
            }
        }
        println!("{} questions with score {}", level_count, current_level_score);
        total_count += level_count;
        println!("Total questions: {}", total_count);
    }

    fn set_bottom_level_limit(&mut self) {

        self.ensure_ordered();

        let min_score = self.questions.first().unwrap().min_score();

        let question_count = self.questions
            .iter()
            .take_while(|q| q.min_score() == min_score)
            .count();
        
        // println!("Asking {} questions with score {}", question_count, min_score);

        self.bottom_limit_index = question_count;
    }

    fn ensure_ordered(&mut self) {
        //if !self.is_ordered_ascending {
        //    self.sort_by_scores(Order::Ascending);
        //}
        self.sort_by_scores(Order::Ascending);
    }
}
