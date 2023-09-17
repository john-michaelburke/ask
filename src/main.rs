use chatgpt::prelude::*;
use clap::Parser;

mod gpt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// OpenAI API key.
    #[arg(long, short, env = "OPENAI_API_KEY")]
    key: Option<String>,

    /// Query
    query: String,
}
    

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let key = args.key.expect("OPENAI_API_KEY not set or `--key` not provided");
    let query = args.query;

    let response = gpt::process_query(key, query).await?;

    println!("\n{}\n", response.message().content);

    Ok(())
}