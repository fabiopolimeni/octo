use std::io::{self, Write};

extern crate crossterm;
extern crate rustyline;

use crossterm::{
    execute,
    style::{self, Stylize},
    cursor,
    terminal,
};

fn main() {
    // Initialize term instance
    let mut stdout = io::stdout();
    
    // Create a new 'readline' instance
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    writeln!(stdout, "{}{}", style::Attribute::Bold, "Welcome to Rustyline!".green()).unwrap();
    execute!(stdout, style::SetAttribute(style::Attribute::Reset)).unwrap();

    loop {

        let readline = rl.readline(": ");
        
        match readline {
            Ok(input) => {
                if input == "/exit" || input == "/quit" {
                    break;
                } else {
                    write!(stdout, "").unwrap();
                    writeln!(stdout, "{}: {}", "󱚠", "Thinking...".italic()).unwrap();
                }
            }
            Err(_) => {
                writeln!(stdout, "").unwrap();
                writeln!(stdout, "{}", "[Error]: Invalid Input.".red().italic()).unwrap();
                break;
            }
        }
    }

    execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0,0), cursor::Show).unwrap();
}
