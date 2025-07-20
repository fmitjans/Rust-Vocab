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
    pub question_levels: Vec<QuestionLevel>,
    pub altered_questions: Vec<Question>,
    pub number_of_wrongs: usize,
}

pub enum Order {
    Ascending,
    Descending,
}

impl QuestionRoster {

    pub fn new(questions: Vec<Question>) -> Self {
        Self {
            questions: questions,
            question_levels: Vec::new(),
            altered_questions: Vec::new(),
            number_of_wrongs: 0,
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

    pub fn interrogate_lowest2(&mut self) {

        if self.question_levels.is_empty() {
            println!("No question levels available.");
            return;
        }

        let min_count = MINIMUM_QUESTION_COUNT;
        let temp_level_index = self.question_levels.iter()
                .position(|level| level.questions.len() >= min_count);

        let level_index = match temp_level_index {
            Some(index) => {
                println!("Selected level with score {}", self.question_levels[index].score);
                index
            },
            None => {
                println!("No level has enough questions (minimum {}). Using the first level.", min_count);
                0
            }
        };

        let mut roster_clone = self.clone();
        for question_index in 0..self.question_levels[level_index].questions.len() {
            let new_question = self.question_levels[level_index].questions[question_index]
                .interrogate(&roster_clone);
            roster_clone.question_levels[level_index].questions[question_index] = new_question;
        }

        self.question_levels = roster_clone.question_levels;
        
    }

    pub fn even_out_scores(&mut self) {

        let lowest_score = self.questions.first().unwrap().min_score();
        for question in self.questions.iter_mut() {
            question.decrease_score(lowest_score);
        }

    }

    pub fn destruct_levels(&mut self) {

        self.questions.clear();

        for level in &self.question_levels {
            for question in &level.questions {
                self.questions.push(question.clone());
            }
        }

        for question in &self.altered_questions {
            self.questions.push(question.clone());
        }

        self.sort_by_scores(Order::Ascending);
    }

    pub fn save(&mut self, file_name: &str) {
        self.destruct_levels();
        self.sort_by_scores(Order::Ascending);
        self.even_out_scores();
        self.print_levels();
        // self.sort_by_scores(Order::Descending);
        save_json(&self.questions, file_name);
    }

    pub fn print_levels(&mut self) {

        let mut total_count = 0;
        for level in &self.question_levels {
            println!("{} questions in level {}", level.length, level.score);
            total_count += level.length;
        }

        println!("Total questions: {}", total_count);
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
