
mod file_handler;
mod params;
use file_handler::FileReader;

fn main() {
    FileReader::load_questions();
}
