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
    /// URL service endpoint
    #[arg(short, long)]
    url: Option<String>,

    /// Model name
    #[arg(short, long)]
    model: Option<String>,

    /// Whether using streaming for a better UX
    /// FIXME: This should exist or not, not being a boolean
    #[arg(short, long)]
    stream: bool,
}

// TODO: Implement commands
// enum Cmds {
//     Exit,
//     User,
//     System,
//     Context,
//     Save,
// }

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
            .url
            .unwrap_or("https://api.openai.com/v1/chat/completions".to_string()),
        &opts.model.unwrap_or("gpt-3.5-turbo".to_string()),
    );

    writeln!(
        stdout,
        "{}{}",
        style::Attribute::Bold,
        "Welcome to Octo!".green()
    )?;

    // Loop through user input
    loop {
        execute!(
            stdout,
            style::SetAttribute(style::Attribute::Reset),
            cursor::EnableBlinking
        )?;

        let input = rl.readline("\n")?;

        if input == "/exit" || input == "/quit" {
            break;
        } else if input.starts_with("/user") {
            writeln!(stdout, "")?;
            execute!(stdout, cursor::SavePosition)?;

            // FIXME: Using animated waiting
            writeln!(stdout, "{}", "Thinking...".italic().blue())?;

            // Assume the worst, prepare terminal style for the error.
            // Because we are not explicitelly handling errors, anything
            // caught after this point will be printed out in bold red.
            execute!(
                stdout,
                style::SetAttribute(style::Attribute::Bold),
                style::SetForegroundColor(style::Color::Red)
            )?;

            if opts.stream {
                let _ = chat
                    .stream(chat::Role::User, &input, |chunk, what| {
                        match what {
                            chat::What::Start => {
                                execute!(
                                    &stdout,
                                    cursor::RestorePosition,
                                    terminal::Clear(terminal::ClearType::FromCursorDown),
                                    style::SetAttribute(style::Attribute::Reset)
                                )
                                .unwrap();
                            }
                            chat::What::Chunk => {
                                // Append text response
                                write!(&stdout, "{}", chunk.italic().blue()).unwrap();

                                // Flush stdout after each chunk.
                                io::stdout().flush().unwrap();
                            }
                            chat::What::Stop => {
                                writeln!(&stdout, "").unwrap();
                            }
                            _ => {}
                        }
                    })
                    .await?;
            } else {
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
    }

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Show
    )?;

    Ok(())
}
