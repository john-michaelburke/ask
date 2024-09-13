use chatgpt::prelude::*;
use clap::Parser;
mod ask;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// OpenAI API key.
    #[arg(long, short, env = "OPENAI_API_KEY")]
    key: Option<String>,

    /// ChatGPT Engine.
    #[arg(
        long,
        short,
        env = "OPENAI_CHATGPT_ENGINE",
        default_value = "gpt-4o-2024-08-06"
    )]
    engine: Option<String>,

    /// Query
    #[arg(required = true, num_args = 1..)]
    query: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let key = args
        .key
        .expect("OPENAI_API_KEY not set or `--key` not provided");
    let static_engine: &'static str = Box::leak(
        args.engine
            .expect(
                "OPENAI_CHATGPT_ENGINE not set or `--engine` not provided; \
        however, the default should have triggered.",
            )
            .into_boxed_str(),
    );
    let engine = ChatGPTEngine::Custom(static_engine);
    let query = args.query.join(" ");

    let response = ask::process_query(key, engine, query).await?;
    for message in response {
        println!("{}", message.content);
    }
    Ok(())
}
