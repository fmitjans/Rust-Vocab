use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AtomicQuestion {
    question: String,
    answer: String,
    score: u32,
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