extern crate nexus;
use nexus::completion;
use nexus::chains::Prompt;
use tokio;
use serde::{Serialize,Deserialize};
use serde_json::{json};
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use crate::database::{Filter, list_points_with_filter};

#[derive(Serialize,Deserialize,Debug)]
pub struct EmbeddingResponse {
    object: String,
    model: String,
    usage: Usage,
    data: Vec<EmbeddingObject>,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Usage {
    prompt_tokens : u32,
    total_tokens : u32,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct EmbeddingObject {
    object : String,
    index : u32,
    embedding : Vec<f32>
}




pub async fn generate_embedding_vector(question: &str) -> Vec<f32> {
    dotenv().ok();
    let client = Client::new();
    let api_key = env::var("OPENAI_API_KEY").expect("Wasnt able to read the key");
    let url = "https://api.openai.com/v1/embeddings";

    let request_body = json!({
        "input": question,
        "model": "text-embedding-3-large"
    });

    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await
        .unwrap()
        .json::<EmbeddingResponse>()
        .await
        .unwrap();

    let embedding = response.data.get(0).unwrap().embedding.clone();
    embedding
}


pub async fn retrieve(filter: &Filter) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let url = "http://localhost:6333/collections";
    match list_points_with_filter(&client, url, "Rug_sage", filter).await {
        Ok(res) => {
            let content: Vec<_> = res
                .result
                .points.iter()
                .map(|page| page.payload.page_content.to_string())
                .collect();
            let content_string = content.join("\n");
            Ok(content_string)
        }
        Err(e) => {
            Err(e)
        }
    }
}


pub async fn summarize_chat_history(text:&str) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("Wasn't able to read the key");

    let chat_history_text = Prompt::new("summarize this text:\n {text}".to_string(),"user".to_string())
        .add_variable("text".to_string(),text.to_string())
        .to_message();

    let system_prompt = Prompt::new("You are a helpful AI assistant that summarizes text in less than 50 words".to_string(), "system".to_string())
        .to_message();

    let messages = vec![system_prompt,chat_history_text];
    let chat_summary = completion(&api_key,messages,0.2).await?;

    Ok(chat_summary)


}


pub async fn chain(question: &str, chat_history: &str, filter: &Filter) -> Result<String, Box<dyn std::error::Error>> {

    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("Wasn't able to read the key");

    let sys = "
            You are an AI critical thinker research assistant specializing in rugs, particularly oriental rugs,
            and all that is relevant to them. Your sole purpose is answer questions based on the given text.
            Your answers are always in HTML which can be readily rendered.
             Dont use ```html at the begining or the end of your answers.
            Please do your best, this is very important to my career.". to_string();
    let sys_prompt = Prompt::new(sys,"system".to_string())
        .to_message();
    let user_prompt = Prompt::new("question: {question} \n {info}, {chat_history}".to_string(),"user".to_string())
        .add_variable("question".to_string(),question.to_string())
        .add_variable("info".to_string(),retrieve(filter).await?)
        .add_variable("chat_history".to_string(), chat_history.to_string())
        .to_message();


    let messages = vec![sys_prompt,user_prompt];

    let response = completion(&api_key,messages,0.5).await?;

    Ok(response)


}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_generate_embedding_vector() {
        dotenv().ok();
        let question = "What is the history of Persian rugs?";
        let embedding = generate_embedding_vector(question).await;
        println!("{:?}", &embedding);

        assert!(!embedding.is_empty(), "Embedding vector should not be empty");
    }

    #[tokio::test]
    async fn test_summarize_chat_history() {
        let chat_history = "This is a long chat history text that needs summarization.";
        let summary = summarize_chat_history(chat_history).await;

        assert!(summary.is_ok(), "Summarize chat history should return Ok");
        let summary_text = summary.unwrap();
        assert!(!summary_text.is_empty(), "Summary should not be empty");
        assert!(summary_text.len() <= 100, "Summary should be 50 words or less");
    }
    #[tokio::test]
    async fn test_retrieve() {
        let question = "What is the symbolism of Boteh in Persian rugs?";
        let filter = Filter {
            limit: 2,
            offset: None,
            must: None,
            should: None,
        };

        let result = retrieve(&filter).await;
        match result {
            Ok(ref results) => {
                assert!(result.is_ok(), "Retrieve function should return Ok");
                println!("Result: {}", results);
            }
            Err(e) => {
                println!("Error occurred: {:?}", e);
                panic!("Retrieve should return Ok, but it returned an error");
            }
        }
    }
    #[tokio::test]
    async fn test_chain() {
        let question = "Explain the importance of rug patterns.";
        let chat_history = "Previous discussions on rug patterns and history.";
        let filter = Filter {
            limit: 2,
            offset: None,
            must: None,
            should: None,
        };
        let response = chain(question, chat_history, &filter).await;
        println!("response: {}", response.as_ref().unwrap());
        assert!(response.is_ok(), "Chain function should return Ok");
        assert!(!response.unwrap().is_empty(), "Chain response should not be empty");

    }
}


