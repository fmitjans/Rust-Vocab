
mod file_handler;
mod params;
mod question_roster;
mod question;
mod terminal;

use file_handler::{FileReader, save_json};
use question_roster::QuestionRoster;

fn main() {
    // myjson::read_json();
    let question_vec = FileReader::load_questions();

    let mut roster = QuestionRoster::new(question_vec);
    roster.shuffle_by_scores();
    roster.interrogate_lowest();

    save_json(&roster.questions, "saved.json");

}


// fn main2() {
//     println!();
// }