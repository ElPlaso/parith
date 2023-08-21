// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::parser::Parser;

mod expression;
mod parser;
pub mod test;

#[tauri::command]
fn run(input: &str) -> String {
    let mut prog = Parser::new(input);

    match prog.parse() {
        Ok(parsed) => match parsed.eval() {
            Ok(result) => {
                return result.to_string();
            }
            Err(error) => {
                return format!("Error evaluating expression: {}", error);
            }
        },
        Err(error) => {
            return format!("Error parsing expression: {}", error);
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
