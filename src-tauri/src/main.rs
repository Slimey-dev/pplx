// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use lazy_static::lazy_static;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::sync::Mutex;
use tauri::{generate_context, Manager};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
  model: String,
  prevent_exit: bool,
}

fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let show = CustomMenuItem::new("show".to_string(), "Show");

  let tray_menu = SystemTrayMenu::new()
    .add_item(show)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  let tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
    .on_window_event(|event| match event.event() {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        let prevent_exit = PREVENT_EXIT.lock().unwrap();
        if *prevent_exit {
          event.window().hide().unwrap();
          api.prevent_close();
        }
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      async_command,
      set_prevent_exit,
      async_config_loader,
      async_config_saver
    ])
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "quit" => {
          std::process::exit(0);
        }
        "show" => {
          let window = app.get_window("main").unwrap();
          window.show().unwrap();
        }
        _ => {}
      },
      SystemTrayEvent::DoubleClick {
        position: _,
        size: _,
        ..
      } => {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
      }
      _ => {}
    })
    .build(generate_context!())
    .expect("error while running tauri application")
    .run(|_app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
        let prevent_exit = PREVENT_EXIT.lock().unwrap();
        if *prevent_exit {
          api.prevent_exit();
        }
      }
      _ => {}
    });
}

lazy_static! {
  static ref CHAT_HISTORY: Mutex<Vec<BTreeMap<&'static str, String>>> = Mutex::new(Vec::new());
}

lazy_static! {
  static ref PREVENT_EXIT: Mutex<bool> = Mutex::new(false);
}

#[tauri::command]
async fn async_config_loader(model: &str, prevent_exit: bool) -> Result<String, String> {
  let config = config_loader(model, prevent_exit)
    .await
    .map_err(|e| e.to_string())?;
  let config_json = serde_json::to_string(&config).map_err(|e| e.to_string())?;
  Ok(config_json)
}

#[tauri::command]
async fn async_config_saver(model: &str, prevent_exit: bool) -> Result<(), String> {
  print!("Saving config...");
  config_saver(model, prevent_exit)
    .await
    .map_err(|e| e.to_string())?;
  print!("Config saved.");
  Ok(())
}

async fn config_loader(
  model: &str,
  prevent_exit: bool,
) -> Result<Config, Box<dyn std::error::Error>> {
  let config_path = "config.json";
  let config: Config;

  if fs::metadata(config_path).is_ok() {
    // Config file exists, load it
    let config_json = fs::read_to_string(config_path)?;
    config = serde_json::from_str(&config_json)?;
  } else {
    // Config file doesn't exist, create it
    config = Config {
      model: model.to_string(),
      prevent_exit,
    };
    let config_json = serde_json::to_string(&config)?;
    let mut file = fs::File::create(config_path)?;
    file.write_all(config_json.as_bytes())?;
  }

  let mut prevent_exit_lock = PREVENT_EXIT.lock().unwrap();
  *prevent_exit_lock = config.prevent_exit;

  Ok(config)
}

async fn config_saver(model: &str, prevent_exit: bool) -> Result<(), Box<dyn std::error::Error>> {
  let config = Config {
    model: model.to_string(),
    prevent_exit,
  };
  let config_json = serde_json::to_string(&config)?;
  let mut file = fs::File::create("config.json")?;
  print!("Saving config: {}", config_json);
  file.write_all(config_json.as_bytes())?;
  Ok(())
}

#[tauri::command]
async fn set_prevent_exit(set_prevent_exit: bool) {
  println!("set_prevent_exit called");
  let mut prevent_exit = PREVENT_EXIT.lock().unwrap();
  *prevent_exit = !set_prevent_exit;
  print!("Prevent exit: {}", *prevent_exit);
}

#[tauri::command]
async fn async_command(
  selected_model: &str,
  input: &str,
  delete_chat_history: bool,
) -> Result<String, String> {
  print!("Selected model: {}", selected_model);
  let result = ai_request(&selected_model, &input, delete_chat_history).await;
  result
}

async fn ai_request(
  selected_model: &str,
  input: &str,
  delete_chat_history: bool,
) -> Result<String, String> {
  dotenv().ok();
  let pplx_key = env::var("PPLX_API_KEY").expect("API_KEY must be set");

  let client = reqwest::Client::new();

  {
    let mut chat_history = CHAT_HISTORY.lock().unwrap();

    if delete_chat_history {
      chat_history.clear();
    }

    if chat_history.is_empty() {
      let mut system_msg_map: BTreeMap<&'static str, String> = BTreeMap::new();
      system_msg_map.insert("role", "system".to_string());
      system_msg_map.insert(
        "content",
        "Be precise and concise. Write your messages in markdown. Write codeblocks in markdown."
          .to_string(),
      );
      chat_history.push(system_msg_map);
    }

    let new_message = input.to_string();

    let mut new_msg_map: BTreeMap<&'static str, String> = BTreeMap::new();
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
      "model": selected_model,
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
    .header(AUTHORIZATION, format!("Bearer {}", pplx_key))
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

    let mut assistant_msg_map: BTreeMap<&'static str, String> = BTreeMap::new();
    assistant_msg_map.insert("role", "assistant".to_string());
    assistant_msg_map.insert("content", message.to_string());

    chat_history.push(assistant_msg_map);
  }

  Ok(message.to_string())
}
