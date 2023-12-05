// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use curl::easy::{Easy, List};
use lazy_static::lazy_static;
use serde_json;
use std::collections::HashMap;
use std::sync::Mutex;

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

lazy_static! {
    static ref CHAT_HISTORY: Mutex<Vec<HashMap<&'static str, String>>> = Mutex::new(Vec::new());
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

    {
        let mut chat_history = CHAT_HISTORY.lock().unwrap();
        if chat_history.is_empty() {
            let mut system_msg_map: HashMap<&'static str, String> = HashMap::new();
            system_msg_map.insert("role", "system".to_string());
            system_msg_map.insert(
                "content",
                "Be precise and concise. Write your messages in markdown. Write codeblocks in markdown.".to_string(),
            );
            chat_history.push(system_msg_map);
        }

        let new_message = input.to_string();

        let mut new_msg_map: HashMap<&'static str, String> = HashMap::new();
        new_msg_map.insert("role", "user".to_string());
        new_msg_map.insert("content", new_message);

        chat_history.push(new_msg_map);
    }

    let chat_history_json = {
        let chat_history = CHAT_HISTORY.lock().unwrap();
        serde_json::to_string(&(*chat_history)).unwrap()
    };

    let payload = format!(
        r#"{{
        "model":"pplx-7b-chat",
        "messages":{},
        "max_tokens":0,
        "temperature":1,
        "top_p":1,
        "top_k":0,
        "stream":false,
        "presence_penalty":0,
        "frequency_penalty":1}}"#,
        chat_history_json
    );

    println!("Payload: {}", payload);

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
    print!("{:?}", json);
    let message = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("");

    {
        let mut chat_history = CHAT_HISTORY.lock().unwrap();

        let mut assistant_msg_map: HashMap<&'static str, String> = HashMap::new();
        assistant_msg_map.insert("role", "assistant".to_string());
        assistant_msg_map.insert("content", message.to_string());

        chat_history.push(assistant_msg_map);
    }

    message.to_string()
}
