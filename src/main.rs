mod conversations_collection;
mod questions_editor;
mod game;

use std::vec::Vec;
use game::*;
use std::env;
use questions_editor::*;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    
    if args.len() > 1 && args[1] == "editor".to_string() {
        QuestionsEditor::start_editor();
    } else {
        // Check if game file exists
        match QuestionsEditor::load_existing_game() {
            Some(conversations) => {
                main_questions_cycle(&conversations);
            },
            None => {
                println!("🎮 Welcome to Text Adventures Engine!");
                println!("📝 No game found. Let's create your first adventure!");
                println!("🚀 Starting the editor...\n");
                QuestionsEditor::start_editor();
            }
        }
    }
}

