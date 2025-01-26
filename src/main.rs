
mod file_handler;
mod params;
mod question_roster;
mod question;
mod terminal;

use file_handler::FileReader;
use question_roster::QuestionRoster;

fn main() {
    // myjson::read_json();
    let question_vec = FileReader::load_questions();

    let mut roster = QuestionRoster::new(question_vec);
    roster.shuffle_by_scores();

    // for e in &roster.questions {
    //     println!("{:#?}", e);
    // }

    // println!("{:#?}", roster.questions[1]);
    // match &mut roster.questions[1] {
    //     question::Question::AtomicQuestion(q) => {
    //         q.interrogate();
    //     },
    //     _ => (),
    // }

    roster.interrogate_lowest();
}


// fn main2() {
//     println!();
// }