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