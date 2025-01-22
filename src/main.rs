
mod file_handler;
mod params;
mod question_roster;
mod question;

use file_handler::FileReader;
use question_roster::QuestionRoster;

fn main() {
    // myjson::read_json();
    let question_vec = FileReader::load_questions();

    let mut roster = QuestionRoster::new(question_vec);
    roster.shuffle_by_scores();

    for e in &roster.questions {
        println!("{:#?}", e);
    }

    println!("Bottom level limit: {}", roster.get_bottom_level_limit());
}
