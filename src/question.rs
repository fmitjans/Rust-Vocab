use serde::{Deserialize, Serialize};
use crate::terminal::Terminal;

type ScoreType = i32;

#[derive(Serialize, Deserialize, Debug)]
pub struct AtomicQuestion {
    question: String,
    answer: String,
    score: ScoreType,
    #[serde(default)]
    note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceQuestion {
    content: Vec<AtomicQuestion>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Question {

    #[serde(rename = "sequence")]
    SequenceQuestion(SequenceQuestion),
    
    #[serde(rename = "atomic")]
    AtomicQuestion(AtomicQuestion),

}

impl Question {
    pub fn min_score(&self) -> ScoreType {
        match self {
            Question::AtomicQuestion(q) => q.score,
            Question::SequenceQuestion(q) => {
                let min_question = q.content
                    .iter()
                    .min_by_key(|aq| aq.score)
                    .unwrap();

                min_question.score
            },
        }
    }

    pub fn interrogate(&mut self) {
        let min_score = self.min_score();
        print!("\x1B[2J\x1B[1;1H"); // clear console

        match self {
            Question::AtomicQuestion(q) => q.interrogate(),
            Question::SequenceQuestion(q) => q.interrogate(min_score),
            }
        }
}

pub enum Answer {
    Correct(String),
    Incorrect(String),
}

impl SequenceQuestion {
    pub fn interrogate(&mut self, min_score: ScoreType) {
        // println!("Sequence question with min score: {}", min_score);

        for atomic_question in &mut self.content {

            // if good score, skip question
            if atomic_question.score != min_score {
                println!();
                println!("Skipping question");
                atomic_question.print_question();
                println!("{}", atomic_question.answer);
                pause_for_key();
                println!();
                continue;
            }

            // if bad score
            atomic_question.interrogate();
        }
    }
}

impl AtomicQuestion {

    pub fn interrogate(& mut self) {
        
        let mut decreased_score_already = false;
        let mut terminal = Terminal::new(); // rustyline terminal

        loop {
            match self.ask(&mut terminal) {
                
                Answer::Correct(a) => {
                    self.give_feedback(a);
                    if !decreased_score_already {
                        self.score += 1;
                    }
                    pause_for_key();
                    break;
                },

                Answer::Incorrect(a) => {
                    if !decreased_score_already {
                        self.score -= 1;
                        decreased_score_already = true;
                    }
                    self.give_feedback(a);
                },
            }
        }
    }
    
    pub fn ask(&mut self, terminal: &mut Terminal) -> Answer {

        println!("Current score: {}", self.score);

        self.print_question();

        let user_answer = terminal.read_line(">> ");

        if user_answer.to_lowercase() == self.answer.to_lowercase() {
            Answer::Correct(user_answer)
        } else {
            Answer::Incorrect(user_answer)
        }
        
    }

    fn print_question(&self) {
        println!("{}", self.question);

        if let Some(note) = &self.note {
            println!("\"{}\"\n", note);
        }
    }

    fn give_feedback(&self, user_answer: String) {
        print_green(&self.answer);
        print!("\n");

        for (char, correct) in user_answer.chars().zip(self.answer.chars()) {
            if char == correct {
                print_green(&char.to_string());
            } else if char.to_lowercase().to_string() == correct.to_lowercase().to_string() {
                print_orange(&char.to_string());
            } else {
                print_red(&char.to_string());
            }
        }

        if user_answer.len() < self.answer.len() {
            print_red(&"...");
        }
        println!();
    }
}

fn pause_for_key() {
    let mut terminal = Terminal::new();
    terminal.read_line("Press Enter to continue...");
}

fn print_red(s: &str) {
    print!("\x1b[31m{}\x1b[0m", s);
}

fn print_orange(s: &str) {
    print!("\x1b[33m{}\x1b[0m", s);
}

fn print_green(s: &str) {
    print!("\x1b[32m{}\x1b[0m", s);
}