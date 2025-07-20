
mod file_handler;
mod params;
mod question_roster;
mod question;
mod terminal;
mod question_level;

use file_handler::{FileReader};
use question_roster::{QuestionRoster};

fn main() {
    let question_vec = FileReader::load_questions();

    let mut roster = QuestionRoster::new(question_vec);
    roster.build_levels();
    roster.print_levels();

    roster.interrogate_lowest2();

    roster.save("saved.json");

}


// fn main2() {
//     println!();
// }