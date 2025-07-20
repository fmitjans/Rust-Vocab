use serde::{Deserialize, Serialize};
use crate::terminal::Terminal;
use crate::question_roster::{QuestionRoster};

pub type ScoreType = i32;

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

pub enum Command {
    Save,
    SkipCorrect,
    SkipNeutral,
    ToggleSkip,
}

pub enum Answer {
    Correct(String),
    Incorrect(String),
    Command(Command),
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

    pub fn interrogate(&self, roster_ref: &QuestionRoster) -> Question {
        let min_score = self.min_score();
        print!("\x1B[2J\x1B[1;1H"); // clear console

        match self {
            Question::AtomicQuestion(q) =>  Question::AtomicQuestion(q.interrogate(roster_ref)),
            Question::SequenceQuestion(q) => Question::SequenceQuestion(q.interrogate(min_score, roster_ref)),
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

fn string_to_command(s: &str) -> Option<Command> {
    match s {
        "1" => Some(Command::Save),
        "2" => Some(Command::SkipNeutral),
        "3" => Some(Command::SkipCorrect),
        "4" => Some(Command::ToggleSkip),
        _ => None,
    }
}

impl SequenceQuestion {
    pub fn interrogate(&self, min_score: ScoreType, roster_ref: &QuestionRoster) -> SequenceQuestion {
        
        let mut new_sequence = SequenceQuestion {
            content: Vec::new(),
        };

        for atomic_question in &self.content {

            // if good score, skip question
            if atomic_question.score != min_score {

                new_sequence.content.push(atomic_question.clone());

                println!();
                println!("Skipping question");
                atomic_question.print_question();
                println!("{}", atomic_question.answer);
                pause_for_key();
                println!();
                continue;
            }

            // if bad score
            let new_atomic = atomic_question.interrogate(roster_ref);
            new_sequence.content.push(new_atomic);
        }

        return new_sequence;
    }
}

impl AtomicQuestion {

    pub fn interrogate(&self, roster_ref: &QuestionRoster) -> AtomicQuestion {
        
        let mut decreased_score_already = false;
        let mut terminal = Terminal::new(); // rustyline terminal

        let mut question_clone = self.clone();
        let unaltered_clone = question_clone.clone();

        loop {
            match question_clone.ask(&mut terminal) {
                
                Answer::Correct(a) => {

                    question_clone.give_feedback(a);

                    if !decreased_score_already {
                        question_clone.score += question_clone.previous_raise;
                        question_clone.previous_raise += 1;
                        println!("Score raised to {}", question_clone.score);
                    }
                    
                    pause_for_key();
                    return question_clone;
                },

                Answer::Incorrect(answer) => {
                    if !decreased_score_already {
                        question_clone.score -= 1;
                        println!("Score lowered to {}", question_clone.score);
                        decreased_score_already = true;
                        question_clone.previous_raise = (question_clone.previous_raise - 1).max(1);
                    }
                    self.give_feedback(answer);
                },

                Answer::Command(command) => {
                    match command {

                        Command::Save => {
                            let mut roster_copy = roster_ref.clone();
                            roster_copy.save("saved.json");
                            println!("Your progress has been saved.");
                        },

                        Command::SkipNeutral => {
                            println!();
                            println!("Skipping question (neutral)");
                            unaltered_clone.print_question();
                            println!("{}", unaltered_clone.answer);
                            pause_for_key();
                            return unaltered_clone;
                        },

                        Command::SkipCorrect => {
                            println!();
                            println!("Skipping question (correct)");

                            question_clone.score += 1;
                            self.print_question();
                            println!("{}", self.answer);

                            pause_for_key();
                            return question_clone;
                        },

                        Command::ToggleSkip => {
                            todo!();
                        },
                    }
                },
            }
        }
    }
    
    pub fn ask(&self, terminal: &mut Terminal) -> Answer {

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