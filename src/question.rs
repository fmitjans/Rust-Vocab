use serde::{Deserialize, Serialize};

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

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SimpleQuestion {
//     #[serde(flatten)]
//     content: AtomicQuestion,
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Question {

    #[serde(rename = "sequence")]
    SequenceQuestion(SequenceQuestion),
    
    #[serde(rename = "atomic")]
    AtomicQuestion(AtomicQuestion),
    
    // #[serde(rename = "simple")]
    // SimpleQuestion(SimpleQuestion),

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
}

enum Answer {
    Correct,
    Incorrect(String),
}

impl AtomicQuestion {

    pub fn interrogate(& mut self) {
        
        let mut decreased_score_already = false;
        print!("\x1B[2J\x1B[1;1H"); // clear console

        loop {
            match self.ask() {
                
                Answer::Correct => {
                    println!("Correct!");
                    self.score += 1;
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
    
    pub fn ask(&mut self) -> Answer {

        println!("Current score: {}\n", self.score);

        println!("Question:\n {}\n", self.question);

        if let Some(note) = &self.note {
            println!("Note: \"{}\"\n", note);
        }

        let user_answer = ask_for_input("Answer: ");

        if user_answer == self.answer {
            Answer::Correct
        } else {
            Answer::Incorrect(user_answer)
        }
        
    }

    fn give_feedback(&self, user_answer: String) {
        println!("Incorrect! The correct answer is: {}\n", self.answer);
        println!("Your answer: {}\n", user_answer);
    }
}

fn ask_for_input(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}