#![allow(dead_code)]

use std::io::{self, Write};

mod chat;
mod open_ai;

use crate::chat::Chat;
use crate::open_ai::OpenAI;

use crossterm::{
    cursor, execute,
    style::{self, Stylize},
    terminal,
};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Opts {
    /// Service endpoint
    #[arg(short, long)]
    service: Option<String>,

    /// Model name
    #[arg(short, long)]
    model: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    // Initialize term instance
    let mut stdout = io::stdout();

    // Create a new 'readline' instance
    let mut rl = rustyline::DefaultEditor::new()?;

    // Instantiate the Chat implementation
    let mut chat = OpenAI::new(
        &opts
            .service
            .unwrap_or("https://api.openai.com/v1/chat/completions".to_string()),
        &opts.model.unwrap_or("gpt-3.5-turbo".to_string()),
    );

    writeln!(
        stdout,
        "{}{}",
        style::Attribute::Bold,
        "Welcome to Octo Garbanzo!".green()
    )?;

    execute!(stdout, style::SetAttribute(style::Attribute::Reset))?;

    // Loop through user input
    loop {
        let input = rl.readline(": ")?;
        if input == "/exit" || input == "/quit" {
            break;
        } else {
            execute!(stdout, cursor::SavePosition)?;
            writeln!(stdout, "\n{}", "Thinking...".italic())?;

            // Assume the worst, prepare terminal style for the error.
            // Because we are not explicitelly handling errors, anything
            // caught after this point will be printed out in bold red.
            execute!(
                stdout,
                style::SetAttribute(style::Attribute::Bold),
                style::SetForegroundColor(style::Color::Red)
            )?;

            let response = chat.message(chat::Role::User, &input).await?;

            // No errors, reset terminal style to print out the response message
            execute!(
                stdout,
                cursor::RestorePosition,
                terminal::Clear(terminal::ClearType::FromCursorDown),
                style::SetAttribute(style::Attribute::Reset)
            )?;

            // Print out the response
            writeln!(stdout, "\n{}\n", response.italic().blue())?;
        }
    }

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Show
    )?;

    Ok(())
}
