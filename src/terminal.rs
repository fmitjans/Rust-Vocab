// let mut rl = rustyline::DefaultEditor::new().unwrap();
// let readline = rl.readline(">> ");
// match readline {
//     Ok(line) => println!("Line: {:?}", line),
//     Err(_) => println!("No input"),
// }

use rustyline::DefaultEditor;

pub struct Terminal {
    rl: DefaultEditor,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            rl: DefaultEditor::new().unwrap(),
        }
    }

    pub fn read_line(&mut self, prompt: &str) -> String {
        match self.rl.readline(prompt) {
            Ok(line) => {
                self.rl.add_history_entry(line.as_str()).unwrap();
                line
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                panic!("Ctrl-C pressed (interrupted)");
            },
            Err(e) => panic!("Error: {}", e),
        }
    }
}