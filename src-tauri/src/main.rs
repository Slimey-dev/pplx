// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use curl::easy::{Easy, List};
use serde_json;

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
fn ai_request(input: &str) -> String {
    let mut easy = Easy::new();
    easy.url("https://api.perplexity.ai/chat/completions")
        .unwrap();
    easy.post(true).unwrap();

    let mut list = List::new();
    list.append("accept: application/json").unwrap();
    list.append("content-type: application/json").unwrap();
    list.append("authorization: Bearer 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")
        .unwrap();
    easy.http_headers(list).unwrap();

    let payload = format!(
        r#"{{
        "model":"pplx-7b-chat",
        "messages":[{{"role":"system","content":"Be precise and concise."}}, {{"role": "user", "content": "{}"}}],
        "max_tokens":0,
        "temperature":1,
        "top_p":1,
        "top_k":0,
        "stream":false,
        "presence_penalty":0,
        "frequency_penalty":1}}"#,
        input
    );

    easy.post_fields_copy(payload.as_bytes()).unwrap();

    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    let json: serde_json::Value = serde_json::from_slice(&data).unwrap();
    let message = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("");
    message.to_string()
}
