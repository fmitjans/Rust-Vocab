
mod file_handler;
mod params;
mod myjson;
mod question_roster;

use file_handler::FileReader;

fn main() {
    // myjson::read_json();
    let question_vec = FileReader::load_questions();
}
