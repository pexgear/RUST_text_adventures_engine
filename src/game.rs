use crate::conversations_collection::*;
use std::io;
use serde::{Serialize, Deserialize};

pub enum AnswerResult {
    Correct(PhraseId),
    //Almost(i16),
    Wrong    
}

#[derive(Serialize, Deserialize)]
pub struct CoversationInteractor(pub ConversationsCollection);

impl CoversationInteractor {
    pub fn new() -> Self {
        Self ( ConversationsCollection {
            phrases: Vec::new(),
            questions: Vec::new(),
        	}
        )
    }

    pub fn add_question(&mut self, question: &str, answer: &str) {
        self.0.phrases.push(Phrase::new(String::from(question)));
        let question_id = &self.0.phrases.len() - 1; 
        self.0.phrases.push(Phrase::new(String::from(answer)));
        let answer_id = &self.0.phrases.len() - 1;

        let mut question = Question {
            question_id: PhraseId::new(question_id),
            answers: vec!(), 
        };

        question.answers.push(Answer { id: PhraseId::new(answer_id), next_question_id: PhraseId::new(0)});

        self.0.questions.push(question);
    }

    pub fn add_end_game(&mut self, end_phrase: &str)
    {
        self.0.phrases.push(Phrase::new(String::from(end_phrase)));
        let question_id = &self.0.phrases.len() - 1; 
        let question = Question {
            question_id: PhraseId::new(question_id),
            answers: vec!(), 
        };

        self.0.questions.push(question);

    }

    pub fn give_answer(&self, answer: &String, question_id: &PhraseId) -> AnswerResult {
        for answ in self.0.questions[question_id.0].answers.iter() {
            if self.0.phrases[answ.id.0].0.trim() == answer.trim() {
                return AnswerResult::Correct(PhraseId::new(answ.next_question_id.0));
            }
        }
        AnswerResult::Wrong
    }

}

pub fn main_questions_cycle (conversations: &CoversationInteractor)
{
    if conversations.0.questions.len() > 0 {
        call_next_answer(&conversations, &PhraseId::new(1));
    }
}

fn call_next_answer(conversations: &CoversationInteractor, question_id: &PhraseId) {
    println!("{}", conversations.0.phrases[conversations.0.questions[question_id.0].question_id.0].0);    
    if conversations.0.questions[question_id.0].answers.len() > 0 {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to readline");
    
        match conversations.give_answer(&guess, question_id) {
            AnswerResult::Correct(next_question_id) => {
                println!("Correct");
                call_next_answer(&conversations, &next_question_id);

            },
            //AnswerResult::Almost(diff) => println!("{} wrong chars", diff),
            AnswerResult::Wrong => println!("ah ah ah!"),
        }
    }
}