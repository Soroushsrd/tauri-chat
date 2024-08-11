// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod utils;
use std::error;
use tauri::Manager;
use crate::utils::{chain, generate_embedding_vector};
use crate::database::{Filter, VectorMatch};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
async fn generate_response(question: String, chat_history: &str) -> Result<String, String> {
    let embedded_question = generate_embedding_vector(&question).await;
    let filter = Filter {
        limit: 2,
        offset: None,
        must: None,
        should: Some(vec![VectorMatch {
            vector: "text_embedding".to_string(),
            value: embedded_question,
            threshold: 0.8
        }])
    };
    match chain(&question, chat_history, &filter).await {
        Ok(response) => Ok(response),
        Err(_) => Err("Cannot process the request".to_string()),
    }
}


// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_response])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
