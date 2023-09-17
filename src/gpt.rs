use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;

const CONVERSATION_HISTORY_FILE: &str = "gpt-conversation.json";

pub async fn process_query(key: String, query: String) -> Result<CompletionResponse> {
    let client = ChatGPT::new(key)?;

    let history_file = std::env::temp_dir().join(CONVERSATION_HISTORY_FILE);

    let mut conversation = if history_file.exists() {
        client.restore_conversation_json(&history_file).await?
    } else {
        client.new_conversation()
    };

    let response = conversation.send_message(query).await?;
    conversation.save_history_json(history_file).await?;

    Ok(response)
}
