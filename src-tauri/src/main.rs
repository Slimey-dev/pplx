// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lazy_static::lazy_static;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{self, json};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");

    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![async_command])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn async_command(input: &str) -> Result<String, String> {
    let result = ai_request(&input).await;
    result
}

lazy_static! {
    static ref CHAT_HISTORY: Mutex<Vec<HashMap<&'static str, String>>> = Mutex::new(Vec::new());
}

async fn ai_request(input: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

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

    let messages: Vec<serde_json::Value> = serde_json::from_str(&chat_history_json).unwrap();

    let payload = json!({
        "model": "pplx-7b-chat",
        "messages": messages,
        "max_tokens": 0,
        "temperature": 1,
        "top_p": 1,
        "top_k": 0,
        "stream": false,
        "presence_penalty": 0,
        "frequency_penalty": 1
    });

    println!("Payload: {:?}", payload);

    println!("Sending request...");

    let send_res = client
        .post("https://api.perplexity.ai/chat/completions")
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(
            AUTHORIZATION,
            "Bearer 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",
        )
        .body(payload.to_string())
        .send()
        .await;

    println!("Request sent. Processing response...");

    let res = match send_res {
        Ok(val) => {
            println!("Response received.");
            val
        }
        Err(err) => {
            println!("Error sending request: {}", err);
            return Err(err.to_string());
        }
    };

    let json_res: Result<serde_json::Value, _> = res.json().await;
    let json = match json_res {
        Ok(val) => {
            println!("JSON response: {}", val.to_string());
            val
        }
        Err(err) => return Err(err.to_string()),
    };

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

    Ok(message.to_string())
}
