#![allow(dead_code)]

use std::io::{self, Write};

mod chat;
mod open_ai;

use crate::chat::Chat;

use crossterm::{
    cursor, execute,
    style::{self, Stylize},
    terminal,
};

use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum Provider {
    OpenAI,
    TogetherAI,
    MistralAI,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Opts {
    /// Provider API to use
    #[arg(value_enum)]
    #[arg(default_value = "open-ai")]
    provider: Provider,

    /// API key, uses <PROVIDER>_API_KEY env var if not provided
    #[arg(short, long)]
    api_key: Option<String>,

    /// URL provider endpoint
    #[arg(short, long)]
    url: Option<String>,

    /// Model name
    #[arg(short, long)]
    model: Option<String>,

    /// Use streaming API for quicker responses
    #[arg(short, long)]
    #[arg(default_value = "false")]
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

    // Initiate chat completion
    let mut chat = match &opts.provider {
        Provider::OpenAI => open_ai::OpenAI::new(
            &opts.api_key.unwrap_or(std::env::var("OPENAI_API_KEY")?),
            &opts
                .url
                .unwrap_or("https://api.openai.com/v1/chat/completions".to_string()),
            &opts.model.unwrap_or("gpt-3.5-turbo-1106".to_string()),
        ),
        Provider::TogetherAI => open_ai::OpenAI::new(
            &opts.api_key.unwrap_or(std::env::var("TOGETHERAI_API_KEY")?),
            &opts
                .url
                .unwrap_or("https://api.together.xyz/v1/completions".to_string()),
            &opts
                .model
                .unwrap_or("mistralai/Mixtral-8x7B-Instruct-v0.1".to_string()),
        ),
        Provider::MistralAI => Err(anyhow!("Not implemented yet"))?,
    };

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
        } else {
            writeln!(stdout, "")?;
            execute!(stdout, cursor::SavePosition)?;

            // FIXME: Using animated waiting
            writeln!(stdout, "{}", "Thinking...".italic().blue())?;

            // Assume the worst, prepare terminal style for errors.
            // Because we are not explicitly handling errors, anything
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
                            chat::StreamState::Start => {
                                // No errors, reset terminal style to print out the response message
                                execute!(
                                    &stdout,
                                    cursor::RestorePosition,
                                    terminal::Clear(terminal::ClearType::FromCursorDown),
                                    style::SetAttribute(style::Attribute::Reset)
                                )
                                .unwrap();
                            }
                            chat::StreamState::Chunk => {
                                // Append text response
                                write!(&stdout, "{}", chunk.italic().blue()).unwrap();

                                // Flush stdout after each chunk.
                                io::stdout().flush().unwrap();
                            }
                            chat::StreamState::Stop => {
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
