use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::question::{Question};
use crate::file_handler::save_json;
use crate::question_level::QuestionLevel;
use std::fmt;

const MINIMUM_QUESTION_COUNT: usize = 3;

#[derive(Debug, Clone)]
pub struct QuestionRoster {
    pub questions: Vec<Question>,
    pub current_question_index: usize,
    pub superior_limit_index: usize,
    pub inferior_limit_index: usize,
    pub question_levels: Vec<QuestionLevel>,
    pub wrong_questions: Vec<Question>,
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
            superior_limit_index: 0,
            inferior_limit_index: 0,
            question_levels: Vec::new(),
            wrong_questions: Vec::new(),
        }
    }

    pub fn build_levels(&mut self) {
        self.shuffle_questions();
        self.sort_by_scores(Order::Ascending);
        self.even_out_scores();
        
        while self.questions.len() > 0 {
            let lowest_score = self.questions.first().unwrap().min_score();
            let mut questions_in_level: Vec<Question> = Vec::new();
            self.questions.retain(|q| {
                if q.min_score() == lowest_score {
                    questions_in_level.push(q.clone());
                    false
                } else {
                    true
                }
            });
            if !questions_in_level.is_empty() {
                let level = QuestionLevel::new(questions_in_level);
                self.question_levels.push(level);
            }
        }

        self.print_levels();
    }

    pub fn shuffle_questions(&mut self) {
        self.questions.shuffle(&mut thread_rng());

    }

    pub fn sort_by_scores(&mut self, order: Order) {
        // self.questions.sort_by(|a, b| a.min_score().cmp(&b.min_score()));
        match order {
            Order::Ascending => {
                self.questions.sort_by(|a, b| a.min_score().cmp(&b.min_score()));
            },
            Order::Descending => {
                self.questions.sort_by(|a, b| b.min_score().cmp(&a.min_score()));
            },
        }
    }

    pub fn interrogate_lowest(&mut self) {

        self.ensure_ordered();

        self.set_bottom_level_limits();

        println!("Commands: 1. Save 2. Skip Neutral 3. Skip Correct 4. Toggle Skip\n");

        if self.inferior_limit_index >= self.superior_limit_index {
            println!("No questions to interrogate");
            return;
        }

        self.current_question_index = self.inferior_limit_index;
        while self.current_question_index >= self.inferior_limit_index  && 
              self.current_question_index < self.superior_limit_index {

            let mut current_question = self.questions[self.current_question_index].clone();

            current_question.interrogate(self);

            self.questions[self.current_question_index] = current_question;

            self.current_question_index += 1;
        }
    }

    pub fn even_out_scores(&mut self) {

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

        let mut total_count = 0;
        for level in &self.question_levels {
            println!("{} questions with score {}", level.length, level.score);
            total_count += level.length;
        }

        println!("Total questions: {}", total_count);
    }

    fn set_bottom_level_limits(&mut self) {

        self.ensure_ordered();

        // let min_score = self.questions.first().unwrap().min_score();
        let mut min_score = 0;

        let mut question_count = 0;
        
        while question_count < MINIMUM_QUESTION_COUNT && min_score < self.questions.last().unwrap().min_score() {
            question_count = self.questions
                .iter()
                .filter(|q| q.min_score() == min_score)
                .count();

            println!("Found {} questions with score {}", question_count, min_score);
            if question_count < MINIMUM_QUESTION_COUNT {
                println!("Not enough questions");
                self.inferior_limit_index += question_count;
            }

            println!("Inferior limit index: {}", self.inferior_limit_index);
            
            min_score += 1;
        }
        
        
        
        self.superior_limit_index = self.inferior_limit_index + question_count;
        println!("Superior limit index: {}", self.superior_limit_index);
        println!();
    }

    fn ensure_ordered(&mut self) {
        self.sort_by_scores(Order::Ascending);
    }
}

impl fmt::Display for QuestionRoster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, level) in self.question_levels.iter().enumerate() {
            writeln!(f, "Level {}: score {}", i + 1, level.score)?;
            for (j, q) in level.questions.iter().enumerate() {
                writeln!(f, "  Question {}: {:?}", j + 1, q)?;
            }
        }
        Ok(())
    }
}
