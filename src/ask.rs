use chatgpt::prelude::*;
use futures_util::StreamExt;

const CONVERSATION_HISTORY_FILE: &str = "gpt-conversation.json";

pub async fn process_query(
    key: String,
    engine: ChatGPTEngine,
    query: String,
) -> Result<Vec<ChatMessage>> {
    let mut client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            .engine(engine)
            .build()
            .unwrap(),
    )?;
    client.config.timeout = std::time::Duration::from_secs(30);

    let history_file = std::env::temp_dir().join(CONVERSATION_HISTORY_FILE);

    let mut conversation = if history_file.exists() {
        client.restore_conversation_json(&history_file).await?
    } else {
        client.new_conversation()
    };

    let mut stream = conversation.send_message_streaming(query).await?;
    let mut output: Vec<ResponseChunk> = Vec::new();
    while let Some(chunk) = stream.next().await {
        match chunk {
            ResponseChunk::Content {
                delta,
                response_index,
            } => {
                output.push(ResponseChunk::Content {
                    delta,
                    response_index,
                });
            }
            other => output.push(other),
        }
    }
    let messages = ChatMessage::from_response_chunks(output);
    conversation.history.push(messages[0].to_owned());
    conversation.save_history_json(history_file).await?;
    Ok(messages)
}
