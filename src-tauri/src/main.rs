// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use curl::easy::{Easy, List};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![ai_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn ai_request() {
    let mut easy = Easy::new();
    easy.url("https://api.perplexity.ai/chat/completions")
        .unwrap();
    easy.post(true).unwrap();

    let mut list = List::new();
    list.append("accept: application/json").unwrap();
    list.append("content-type: application/json").unwrap();
    easy.http_headers(list).unwrap();

    let payload = r#"{
        "model":"llama-2-70b-chat",
        "messages":[{"role":"system","content":"string"}],
        "max_tokens":0,
        "temperature":1,
        "top_p":1,
        "top_k":0,
        "stream":false,
        "presence_penalty":0,
        "frequency_penalty":1}"#;

    easy.post_fields_copy(payload.as_bytes()).unwrap();
}
