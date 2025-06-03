use anyhow::Result;
use clap::Parser;
mod ask;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// OpenAI or Gemini API key.
    #[arg(long, short, env = "ASK_API_KEY")]
    key: Option<String>,

    /// OpenAI or Gemini Model Engine.
    #[arg(long, short, env = "MODEL_ENGINE", default_value = "gpt-4o-2024-08-06")]
    engine: Option<String>,

    /// Query to send to the model.
    #[arg(required = true, allow_hyphen_values = true, trailing_var_arg = true)]
    query: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let key = args
        .key
        .expect("ASK_API_KEY not set or `--key` not provided");
    let static_engine: &'static str = Box::leak(
        args.engine
            .expect(
                "MODEL_ENGINE not set or `--engine` not provided; \
        however, the default should have triggered.",
            )
            .into_boxed_str(),
    );
    let query = args.query.join(" ");
    let response = ask::process_query(key, static_engine, query).await?;
    for message in response {
        println!("{}", message);
    }
    Ok(())
}
