use anyhow::Result;
use chatgpt::prelude::*;
use futures_util::StreamExt;
use gemini_rs::Client as gemini_client;
use std::time::Duration;

const CONVERSATION_HISTORY_FILE: &str = "ask-conversation.json";

pub enum ModelEngine {
    OpenAI(ChatGPTEngine),
    Gemini(&'static str),
}

fn deduce_model_engine(engine: &'static str) -> ModelEngine {
    if engine.starts_with("gemini") {
        ModelEngine::Gemini(engine)
    } else if engine.starts_with("gpt") {
        ModelEngine::OpenAI(ChatGPTEngine::Custom(engine))
    } else {
        panic!("Unsupported model engine: {}", engine);
    }
}

trait ModelClient {
    fn client(key: String, engine: ModelEngine) -> Result<Box<Self>>;
    async fn send_message(self, query: String) -> Result<Vec<String>>;
}

struct GPTClient(ChatGPT);
impl ModelClient for GPTClient {
    fn client(key: String, engine: ModelEngine) -> Result<Box<Self>> {
        let engine = if let ModelEngine::OpenAI(engine) = engine {
            engine
        } else {
            panic!("Unsupported model engine for GPTClient");
        };
        Ok(Box::new(GPTClient(ChatGPT::new_with_config(
            key,
            ModelConfigurationBuilder::default()
                .engine(engine)
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap(),
        )?)))
    }

    async fn send_message(self, query: String) -> Result<Vec<String>> {
        let history_file = std::env::temp_dir().join(format!("gpt-{CONVERSATION_HISTORY_FILE}"));

        let mut conversation = if history_file.exists() {
            self.0.restore_conversation_json(&history_file).await?
        } else {
            self.0.new_conversation()
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
        Ok(messages.into_iter().map(|m| m.content).collect())
    }
}

struct GeminiClient(gemini_client, String);
impl ModelClient for GeminiClient {
    fn client(key: String, engine: ModelEngine) -> Result<Box<Self>> {
        if let ModelEngine::Gemini(engine) = engine {
            Ok(Box::new(GeminiClient(
                gemini_client::new(key),
                engine.to_string(),
            )))
        } else {
            panic!("Unsupported model engine for GeminiClient");
        }
    }

    async fn send_message(self, query: String) -> Result<Vec<String>> {
        let mut chat = self.0.chat(&self.1);
        let history_file = std::env::temp_dir().join(format!("gemini-{CONVERSATION_HISTORY_FILE}"));

        // Load or create conversation history by reading json from file
        if history_file.exists() {
            // read raw json from file
            let history_json: Vec<gemini_rs::types::Content> =
                tokio::fs::read_to_string(&history_file)
                    .await?
                    .split('\n')
                    .filter_map(|line| serde_json::from_str(line).ok())
                    .collect();
            chat.history_mut().extend(history_json);
        }
        // Send the query to Gemini
        let response = chat.send_message(&query).await?;
        let messages = response
            .candidates
            .into_iter()
            .flat_map(|candidate| candidate.content.parts[0].text.clone())
            .collect::<Vec<_>>();
        // Overwrite the conversation history to file
        let history = chat
            .history()
            .iter()
            .map(|content| serde_json::to_string(content).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        tokio::fs::write(&history_file, history).await?;
        Ok(messages)
    }
}

pub async fn process_query(
    key: String,
    static_engine: &'static str,
    query: String,
) -> Result<Vec<String>> {
    let engine = deduce_model_engine(static_engine);
    let messages = match engine {
        ModelEngine::OpenAI(_) => {
            let client = GPTClient::client(key, engine)?;
            client.send_message(query).await
        }
        ModelEngine::Gemini(_) => {
            let client = GeminiClient::client(key, engine)?;
            client.send_message(query).await
        }
    }?;
    Ok(messages)
}
