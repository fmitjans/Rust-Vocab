
mod file_handler;
mod params;
mod question_roster;
mod question;

use file_handler::FileReader;

fn main() {
    // myjson::read_json();
    let question_vec = FileReader::load_questions();
    for e in question_vec {
        println!("{:#?}", e);
    }
}
