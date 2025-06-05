use crate::conversations_collection::{Phrase, Question, Answer};
use std::fs::File;
use crate::game::*;

use colored::*;

use std::io::{self, prelude::*};

#[derive(PartialEq)]
pub enum EditorMode {
	None,
	Edit,
	Next,
	Prev,
	Jump,
	List,
	New,
}

enum Context {
	Question,
	Answer,
} 

pub struct QuestionsEditorSettings {
	current_question_index: usize,
	mode: EditorMode,
}

impl QuestionsEditorSettings {
	fn new() -> Self {
		Self{
			current_question_index : 0,
			mode : EditorMode::None,
		}
	}

	fn set_current_question_index(& mut self, id: &PhraseId)
	{
		self.current_question_index = id.0;
	}

	fn reset_mode(& mut self)
	{
		self.mode = EditorMode::None;
	}
}


impl Question {
	fn add_answer(&mut self, answer_id: &PhraseId, next_id: &PhraseId) {
        self.answers.push(Answer { id: PhraseId(answer_id.0), next_question_id: PhraseId(next_id.0)});
	}
}

pub struct QuestionsEditor;

use crate::conversations_collection::PhraseId;

impl QuestionsEditor {
	
	fn new() -> Self {
		Self{}
	}

	fn init(&self) {
		println!("Starting the questions editor...");
		println!("...questions editor ready!");
		println!("\n---------------------------\n");
	}

	pub fn start_editor() {
		let editor = QuestionsEditor::new();
		editor.init();

		let mut conversations = QuestionsEditor::load_or_create_default_game();
		let mut editor_settings = QuestionsEditorSettings::new();
		editor.view_last_question(&mut conversations, &mut editor_settings);

		let j = serde_json::to_string(&conversations).unwrap();

		QuestionsEditor::write_to_file(j.as_bytes()).unwrap();
	}

	fn view_last_question(&self, mut conversations : &mut CoversationInteractor,mut settings: &mut QuestionsEditorSettings) {
		let index = conversations.0.questions.len() - 1;
		self.view_question(&mut conversations, &PhraseId(index), &mut settings);
	}

	fn view_question(&self,mut conversations : &mut CoversationInteractor, question_index: &PhraseId, mut settings: &mut QuestionsEditorSettings) {
		if conversations.0.questions.len() <= question_index.0 {
			println!("This question id is out of bounds")
		} else {
			settings.set_current_question_index(&question_index);
			
			let phrase_index = &conversations.0.questions[question_index.0].question_id;
			

	        if settings.mode == EditorMode::None {
	        	println!("{}_{} Q - {}", question_index.0.to_string().cyan(), phrase_index.0.to_string().cyan(), conversations.0.phrases[phrase_index.0].0.yellow());
				for answ in conversations.0.questions[question_index.0].answers.iter() {
					println!("{}_{} A - {}", question_index.0.to_string().cyan(), answ.id.0.to_string().cyan(), conversations.0.phrases[answ.id.0].0);
		        	println!("↓↓↓↓↓↓↓↓↓↓");
					println!("{}_{} N - {}", question_index.0.to_string().cyan(), answ.next_question_id.0.to_string().cyan(), conversations.0.phrases[conversations.0.questions[answ.next_question_id.0].question_id.0].0);
					println!("---------------\n");
						
		        }
			        println!("{}", "\nActions: --edit  --next  --prev".blue());
			        self.extract_command(&self.wait_for_input(), &mut settings, Context::Question);
	        }

	        match &settings.mode {
		        EditorMode::Edit => {
		        	println!("\nWe are in EDIT mode!\nWhat do you want to edit?\n(Input the correspondant number)");
		        	let mut count : u16 = 0;
		        	println!("{} Q - {}", count.to_string().cyan(), conversations.0.phrases[phrase_index.0].0.yellow());
					for answ in conversations.0.questions[question_index.0].answers.iter() {
		        		count += 1;
						println!("{} A - {}", count.to_string().cyan(), conversations.0.phrases[answ.id.0].0);
			        	println!("↓↓↓↓↓↓↓↓↓↓");
						println!("{} N - {}", count.to_string().cyan(), conversations.0.phrases[answ.next_question_id.0].0);
						println!("---------------\n");
			        }
			        count += 1;
					println!("{} +++ Add another answer...", count.to_string().cyan());

			        let index = self.wait_for_input().parse::<usize>().unwrap();
			        self.edit_phrase(&mut conversations, question_index, &PhraseId(index));
		        	settings.reset_mode();
		        },
		        EditorMode::Prev => {
		        	let new_index = PhraseId(question_index.0 - (if question_index.0 >= 1 { 1 } else { 0 }));
		        	settings.reset_mode();
		        	self.view_question(&mut conversations, &new_index, &mut settings);
		        },
		        EditorMode::Next => {
		        	let new_index = PhraseId(question_index.0 + (if conversations.0.questions.len() > (question_index.0 + 1) { 1 } else { 0 }));
		        	settings.reset_mode();
		        	self.view_question(&mut conversations, &new_index, &mut settings);
		        },
		        __ => {
			        println!("\nActions: --edit  --next  --prev");
			        self.extract_command(&self.wait_for_input(), &mut settings, Context::Question);
			        self.view_question(&mut conversations, &question_index, &mut settings);
		        },
	    
	        }
		}
	}

	fn edit_phrase (&self, mut conversations : &mut CoversationInteractor, question_index : &PhraseId, partial_index: &PhraseId) {
		//println!("\nWe are in EDIT mode!\nWhat do you want to edit?\n(Input the correspondant number)");
    	let count : usize = conversations.0.questions[question_index.0].answers.len();
    	if partial_index.0 == 0 {
			println!("{} Q - {}", count, conversations.0.phrases[conversations.0.questions[question_index.0].question_id.0].0);
			println!("Input new question for replacement...");
			let replacement = self.wait_for_input();
    		conversations.0.phrases[conversations.0.questions[question_index.0].question_id.0].0 = replacement;
		} else if partial_index.0 >= 1 && partial_index.0 <= count {
			self.edit_answer (&mut conversations, question_index, partial_index);
		} else {
    		println!("Adding another answer");
    		self.add_new_answer(&mut conversations, question_index, partial_index);
    	}
		self.save_conversations(&mut conversations);
	}

	fn edit_answer (&self, mut conversations : &mut CoversationInteractor, question_index : &PhraseId, partial_index: &PhraseId) {
		let mut count: usize = 0;
		let questions_count = conversations.0.questions.len();
		for n in 0..conversations.0.questions[question_index.0].answers.len() {
    		count += 1;
    		if count == partial_index.0 {
				println!("{} A - {}", count, conversations.0.phrases[conversations.0.questions[question_index.0].answers[n].id.0].0);
	        	println!("^   ^   ^");
				println!("{} N - {}", count, conversations.0.phrases[conversations.0.questions[conversations.0.questions[question_index.0].answers[n].next_question_id.0].question_id.0].0);
				println!("\n Insert new answer...",);
				let replacement = self.wait_for_input();
				println!("\n Insert next question index",);
				println!("{}", "--ls --nw".blue());
				let mut new_index : String = "".to_string();
				let mut settings = QuestionsEditorSettings::new();
				while (&new_index).is_empty() {
					new_index = self.wait_for_input();
					self.extract_command(&new_index, &mut settings, Context::Answer);
					match &settings.mode {
						EditorMode::New => {
							conversations.0.questions[question_index.0].answers[n].next_question_id.0 = self.add_new_question(&mut conversations);
						},
						EditorMode::List => {
							new_index = "".to_string();
							settings.mode = EditorMode::None;
							self.print_questions_list(&conversations);
						},
						EditorMode::None => {
							let index : usize = (&new_index).parse::<usize>().unwrap();
							if index < questions_count {
								conversations.0.questions[question_index.0].answers[n].next_question_id.0 = index;
							} else {
								eprintln!("{}", "The index is out of bounds".red());
								new_index = "".to_string();
							} 
						},
						__ => eprintln!("Mode not handled"),
					}
				}
				if !(&replacement).is_empty() {
					conversations.0.phrases[conversations.0.questions[question_index.0].answers[n].id.0].0 = replacement.clone();
				}
    		}
        }
	}

	fn add_new_answer (&self, conversations : &mut CoversationInteractor, question_index : &PhraseId, _partial_index: &PhraseId) {
		println!("\n Insert new answer...",);
		let replacement = self.wait_for_input();
		println!("\n Insert next question index",);
		
		let new_index = self.wait_for_input().parse::<usize>().unwrap();
		conversations.0.phrases.push(Phrase::new(replacement));

		conversations.0.questions[question_index.0].add_answer(&PhraseId(conversations.0.phrases.len() - 1), &PhraseId(new_index));
	}

	fn add_new_question (&self, conversations : &mut CoversationInteractor) -> usize {
		println!("{}", "You are adding a new question:");
		let new_question = self.wait_for_input();
		conversations.0.phrases.push(Phrase(new_question));
		let phrase_index = conversations.0.phrases.len() - 1;
		conversations.0.questions.push(Question{
			question_id : PhraseId(phrase_index),
			answers: vec!(),
		});
		let mut settings = QuestionsEditorSettings::new();
		let question_index = conversations.0.questions.len() - 1;
		self.view_question(conversations, &PhraseId(conversations.0.questions.len() - 1), &mut settings);

		question_index
	}

	fn wait_for_input (&self) -> String {
		let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to readline");
        guess.trim().to_string()
	}

	fn extract_command (&self, str: &String, settings : &mut QuestionsEditorSettings, context: Context) {
		match context {
			Context::Question => {
				if str.len() > 5 {
					let slice = &str[..6];
					match slice {
						"--edit" => settings.mode = EditorMode::Edit,
						"--next" => settings.mode = EditorMode::Next,
						"--prev" => settings.mode = EditorMode::Prev,
						"--jump" => settings.mode = EditorMode::Jump,
						__ => eprintln!("Command not recognized"),
					}	
				} else {
					eprintln!("Command not recognized");			
				}
			},
			Context::Answer => {
				if str.len() > 3 {
					let slice = &str[..4];
					match slice {
						"--ls" => settings.mode = EditorMode::List,
						"--nw" => settings.mode = EditorMode::New,
						__ => (),
					}	
				}
			},
		}
	}

	fn print_questions_list (&self, conversations : &CoversationInteractor) {
		for i in 0..conversations.0.questions.len() {
			let question_index = conversations.0.questions[i].question_id.0.clone();
			println!("{} - {}", i.to_string().cyan(), conversations.0.phrases[question_index].0);
		}
	}

	fn save_conversations (&self, conversations : &mut CoversationInteractor) {
		let j = serde_json::to_string(&conversations).unwrap();
		QuestionsEditor::write_to_file(j.as_bytes()).unwrap();
	}

	pub fn read_from_file() -> std::io::Result<String> {
		let mut file = File::open("game_config.txt")?;
		let mut contents = String::new();
	    file.read_to_string(&mut contents)?;
	    //assert_eq!(contents, "Hello, world!");
	    Ok(contents)
	}

	pub fn load_or_create_default_game() -> CoversationInteractor {
		match QuestionsEditor::read_from_file() {
			Ok(contents) => {
				match serde_json::from_str(&contents) {
					Ok(conversations) => conversations,
					Err(_) => {
						println!("Warning: Invalid game_config.txt format. Creating default game...");
						QuestionsEditor::create_default_game()
					}
				}
			},
			Err(_) => {
				println!("No game_config.txt found. Creating default game...");
				QuestionsEditor::create_default_game()
			}
		}
	}

	pub fn create_default_game() -> CoversationInteractor {
		let mut conversations = CoversationInteractor::new();

		// Create a simple default game
		conversations.add_end_game("Welcome to Text Adventures Engine!");
		conversations.add_question("What would you like to do?", "Start the editor");
		conversations.add_question("Great choice! Use 'cargo run editor' to create your adventure.", "Continue");

		// Save the default game
		let j = serde_json::to_string(&conversations).unwrap();
		QuestionsEditor::write_to_file(j.as_bytes()).unwrap();
		println!("Default game created! Use 'cargo run editor' to customize it.");

		conversations
	}

	fn write_to_file(stream: &[u8]) -> std::io::Result<()> {
		let mut file = File::create("game_config.txt")?;
		file.write_all(stream).unwrap();
	    Ok(())
	}
}
