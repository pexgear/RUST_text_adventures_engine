use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Phrase(pub String);
impl Phrase {
	pub fn new(string: String) -> Self {
		Self(string)
	}
}

#[derive(Serialize, Deserialize)]
pub struct PhraseId(pub usize);

impl PhraseId {
	pub fn new(id: usize) -> Self {
		Self(id)
	}
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub id: PhraseId,
    pub next_question_id: PhraseId
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub question_id: PhraseId,
    pub answers: Vec<Answer>,
}

#[derive(Serialize, Deserialize)]
pub struct ConversationsCollection {
    pub phrases: Vec<Phrase>,
    pub questions: Vec<Question>,
}

impl ConversationsCollection {
	pub fn new() -> Self { 
		Self {
			phrases: vec![],
			questions: vec![],
		}
	}
}
