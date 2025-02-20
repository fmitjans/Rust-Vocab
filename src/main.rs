
mod file_handler;
mod params;
mod question_roster;
mod question;
mod terminal;

use file_handler::{FileReader, save_json};
use question_roster::{QuestionRoster, Order};

fn main() {
    let question_vec = FileReader::load_questions();

    let mut roster = QuestionRoster::new(question_vec);
    roster.shuffle_order();
    roster.sort_by_scores(Order::Ascending);
    roster.even_out_scores();
    roster.interrogate_lowest();

    save_json(&roster.questions, "saved.json");

}


// fn main2() {
//     println!();
// }