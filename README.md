# Text Adventures Engine

A Rust-based interactive text adventure game engine that allows you to create and play question-and-answer style adventure games. The engine features both a game runtime and an interactive editor for creating your own adventures.

## Features

- **Interactive Game Engine**: Play text-based adventures with branching storylines
- **Built-in Editor**: Create and modify games using an intuitive command-line editor
- **JSON-based Game Format**: Games are stored in a simple, readable JSON format
- **Colorized Output**: Enhanced visual experience with colored terminal output
- **Serialization Support**: Save and load game configurations seamlessly

## Installation

Make sure you have [Rust](https://rustup.rs/) installed on your system.

1. Clone the repository:
   ```bash
   git clone https://github.com/pexgear/RUST_text_adventures_engine.git
   cd RUST_text_adventures_engine
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Playing a Game

To start playing an existing adventure:

```bash
cargo run
```

The engine will load the game configuration from `game_config.txt` and start the interactive experience.

### Using the Editor

To create or modify adventures, use the editor mode:

```bash
cargo run editor
```

The editor provides several commands:
- `--edit`: Edit questions and answers
- `--next`: Navigate to the next question
- `--prev`: Navigate to the previous question
- `--ls`: List all questions
- `--nw`: Create a new question

### Game Format

Games are structured as a collection of questions and answers:
- Each **question** can have multiple **answers**
- Each **answer** leads to the next question in the story
- Questions with no answers serve as ending points

Example game flow:
```
Question: "How old are you?"
├── Answer: "Young" → Next Question: "What's your favorite color?"
└── Answer: "Old" → End Game

Question: "What's your favorite color?"
├── Answer: "Blue" → End Game
└── Answer: "Red" → End Game
```

## Project Structure

```
src/
├── main.rs                     # Entry point and argument handling
├── game.rs                     # Core game engine and logic
├── questions_editor.rs         # Interactive editor implementation
└── conversations_collection/   # Data structures for questions and answers
    └── mod.rs
```

## Dependencies

- `serde` & `serde_json`: JSON serialization
- `colored`: Terminal color output
- `text_io`: Text input utilities
- `fstream`: File stream operations

## Contributing

**Contributions are welcome!** 🎉

Feel free to:
- 🐛 **Report bugs** by creating an issue
- 💡 **Suggest features** or improvements
- 📝 **Submit pull requests** with fixes or enhancements
- 📖 **Improve documentation**
- 🎮 **Share your game creations**

No need to fork - you can contribute directly! Just:
1. Create an issue to discuss your idea (optional but recommended)
2. Clone the repository and create a feature branch
3. Make your changes and test them with `cargo test`
4. Submit a pull request

All contributions, big or small, are appreciated!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*Happy adventuring! 🎮*
