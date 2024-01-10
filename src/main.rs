#![allow(dead_code)]

use std::io::{self, Write};

mod chat;
mod conversation;

use crate::conversation::{Conversation, Role, State};

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
    Gemini,
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

// TODO - Implement commands
// enum Cmds {
//     Exit,
//     User,
//     System,
//     Context,
//     Save,
//     Load,
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
        Provider::OpenAI => chat::Chat::new(
            &opts.api_key.unwrap_or(std::env::var("OPENAI_API_KEY")?),
            &opts
                .url
                .unwrap_or("https://api.openai.com/v1/chat/completions".to_string()),
            &opts.model.unwrap_or("gpt-3.5-turbo-1106".to_string()),
            &chat::Settings::default(),
        ),
        Provider::TogetherAI => chat::Chat::new(
            &opts.api_key.unwrap_or(std::env::var("TOGETHERAI_API_KEY")?),
            &opts
                .url
                .unwrap_or("https://api.together.xyz/v1/chat/completions".to_string()),
            &opts
                .model
                .unwrap_or("mistralai/Mixtral-8x7B-Instruct-v0.1".to_string()),
            &chat::Settings::default(),
        ),
        Provider::MistralAI => chat::Chat::new(
            &opts.api_key.unwrap_or(std::env::var("MISTRALAI_API_KEY")?),
            &opts
                .url
                .unwrap_or("https://api.mistral.ai/v1/chat/completions".to_string()),
            &opts.model.unwrap_or("mistral-medium".to_string()),
            &chat::Settings::default(),
        ),
        Provider::Gemini => Err(anyhow!("Gemini provider not implemented yet!"))?,
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

        let input = rl.readline("\n")?.trim().to_string().to_owned();

        if input == "/exit" || input == "/quit" {
            break;
        } else {
            writeln!(stdout, "")?;
            execute!(stdout, cursor::SavePosition)?;

            // FIXME - Using animated waiting
            writeln!(stdout, "{}", "Thinking...".italic().blue())?;

            // Assume the worst, prepare terminal style for errors.
            // We are not handling errors, jut bubble them up, therefore,
            // anything caught after this point will be printed out in bold
            // red, but it is handled.
            execute!(
                stdout,
                style::SetAttribute(style::Attribute::Bold),
                style::SetForegroundColor(style::Color::Red)
            )?;

            let _ = chat
                .build(Role::User, &input)
                .execute(|state| {
                    match state {
                        State::Start => {
                            // No errors, reset terminal style to print out the response message
                            execute!(
                                &stdout,
                                cursor::RestorePosition,
                                terminal::Clear(terminal::ClearType::FromCursorDown),
                                style::SetAttribute(style::Attribute::Reset)
                            )
                            .unwrap();
                        }
                        State::Message(text) => {
                            // Append text response
                            write!(&stdout, "{}", text.italic().blue()).unwrap();

                            // Flush stdout after each chunk.
                            io::stdout().flush().unwrap();
                        }
                        State::Stop | State::Done => {
                            writeln!(&stdout, "").unwrap();
                        }
                        _ => {}
                    }
                })
                .await?;
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
