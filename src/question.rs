use serde::{Deserialize, Serialize};
use crate::terminal::Terminal;
use crate::question_roster::{QuestionRoster};

type ScoreType = i32;
fn score_zero() -> ScoreType {
    0
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AtomicQuestion {
    question: String,
    #[serde(default)] // Option<String> implements Default None
    note: Option<String>,
    answer: String,
    score: ScoreType,
    #[serde(default = "score_zero")]
    previous_raise: ScoreType,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceQuestion {
    content: Vec<AtomicQuestion>
}

#[derive(Clone)]
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

    pub fn interrogate(&mut self, roster_ref: &mut QuestionRoster) {
        let min_score = self.min_score();
        print!("\x1B[2J\x1B[1;1H"); // clear console

        match self {
            Question::AtomicQuestion(q) => q.interrogate(roster_ref),
            Question::SequenceQuestion(q) => q.interrogate(min_score, roster_ref),
            }
        }

    pub fn decrease_score(&mut self, amount: ScoreType) {
        match self {
            Question::AtomicQuestion(q) => q.score -= amount,
            Question::SequenceQuestion(q) => {
                for atomic_question in &mut q.content {
                    atomic_question.score -= amount;
                }
            },
        }
    }
}

pub enum Command {
    Save,
    SkipCorrect,
    SkipNeutral,
    ToggleSkip,
}

fn string_to_command(s: &str) -> Option<Command> {
    match s {
        "1" => Some(Command::Save),
        "2" => Some(Command::SkipNeutral),
        "3" => Some(Command::SkipCorrect),
        "4" => Some(Command::ToggleSkip),
        _ => None,
    }
}

pub enum Answer {
    Correct(String),
    Incorrect(String),
    Command(Command),
}

impl SequenceQuestion {
    pub fn interrogate(&mut self, min_score: ScoreType, roster_ref: &mut QuestionRoster) {
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
            atomic_question.interrogate(roster_ref);
        }
    }
}

impl AtomicQuestion {

    pub fn interrogate(& mut self, roster_ref: &mut QuestionRoster) {
        
        let mut decreased_score_already = false;
        let mut terminal = Terminal::new(); // rustyline terminal

        loop {
            match self.ask(&mut terminal) {
                
                Answer::Correct(a) => {
                    self.give_feedback(a);
                    if !decreased_score_already {
                        self.previous_raise += 1;
                        self.score += self.previous_raise;
                        println!("Score raised to {}", self.score);
                    }
                    pause_for_key();
                    break;
                },

                Answer::Incorrect(answer) => {
                    if !decreased_score_already {
                        self.score -= 1;
                        decreased_score_already = true;
                        self.previous_raise = (self.previous_raise - 1).max(0);
                    }
                    self.give_feedback(answer);
                },

                Answer::Command(command) => {
                    match command {

                        Command::Save => {
                            let mut roster_copy = roster_ref.clone();
                            roster_copy.even_out_scores();
                            roster_copy.save("saved.json");
                            println!("Your progress has been saved.");
                        },

                        Command::SkipNeutral => {
                            println!();
                            println!("Skipping question (neutral)");
                            self.print_question();
                            println!("{}", self.answer);
                            pause_for_key();
                            break;
                        },

                        Command::SkipCorrect => {
                            println!();
                            println!("Skipping question (correct)");
                            self.score += 1;
                            self.print_question();
                            println!("{}", self.answer);
                            pause_for_key();
                            break;
                        },

                        Command::ToggleSkip => {
                            todo!();
                        },
                    }
                },
            }
        }
    }
    
    pub fn ask(&mut self, terminal: &mut Terminal) -> Answer {

        println!();
        println!("Current score: {}", self.score);
        println!("Previous raise: {}", self.previous_raise);

        self.print_question();

        let user_answer = terminal.read_line(">> ");

        if let Some(command) = string_to_command(&user_answer) {
            return Answer::Command(command);
        }

        if user_answer.to_lowercase() == self.answer.to_lowercase() {
            Answer::Correct(user_answer)
        } else {
            Answer::Incorrect(user_answer)
        }
        
    }

    fn print_question(&self) {
        println!("{}", self.question);

        if let Some(note) = &self.note {
            println!("\"{}\"", note);
        }
        println!();
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