use chatgpt::prelude::*;
use clap::Parser;
mod ask;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// OpenAI API key.
    #[arg(long, short, env = "OPENAI_API_KEY")]
    key: Option<String>,

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
    let query = args.query.join(" ");

    let response = ask::process_query(key, query).await?;
    for message in response {
        println!("{}", message.content);
    }
    Ok(())
}
