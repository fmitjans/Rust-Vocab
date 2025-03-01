
mod file_handler;
mod params;
mod question_roster;
mod question;
mod terminal;

use file_handler::{FileReader};
use question_roster::{QuestionRoster, Order};

fn main() {
    let question_vec = FileReader::load_questions();

    let mut roster = QuestionRoster::new(question_vec);
    roster.shuffle_questions();
    roster.sort_by_scores(Order::Ascending);
    roster.even_out_scores();
    roster.print_levels();
    roster.interrogate_lowest();

    roster.save("saved.json");

}


// fn main2() {
//     println!();
// }