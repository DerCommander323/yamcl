use std::{io::Error, fs};

use serde_json::{Value, json};
use tauri::api::path::config_dir;

use crate::get_config_dir;

use super::auth_structs::{MCAccount, AccountList, MCProfile};



const ACCOUNT_FILE_NAME: &str = "accounts.json";


fn read_accounts_safe() -> AccountList {
    let accounts_path = get_config_dir().join(ACCOUNT_FILE_NAME);
    if let Ok(file) = fs::read_to_string(&accounts_path) {
        if let Ok(account_list) = serde_json::from_str(&file) {
            return account_list
        }
    }
    let fallback_list = AccountList {
        accounts: Vec::new(),
        selected_index: None,
    };

    if !accounts_path.exists() {
        if let Some(parent) = accounts_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create config directory")
        }
    }
    fs::write(accounts_path, serde_json::to_string_pretty(&fallback_list).unwrap()).expect("Failed to write to accounts file");
    fallback_list
}

pub fn load_accounts() -> Vec<MCAccount> {
    read_accounts_safe().accounts
}

#[tauri::command]
pub fn get_accounts() -> Vec<MCProfile> {
    load_accounts().into_iter().map(|acc| {
        acc.mc_profile
    }).collect()
}

pub fn save_new_account(account: MCAccount) -> Result<(), Error> {
    let mut accounts = load_accounts();
    accounts.push(account);
    set_selected_index((accounts.len()-1).try_into().unwrap_or(0));
    save_accounts(accounts)?;
    Ok(())
}

#[tauri::command]
pub fn remove_account(index: usize) {
    let mut accounts = load_accounts();

    accounts.remove(index);
    save_accounts(accounts).unwrap();
}

pub fn get_active_account() -> Option<MCAccount> {
    let accounts = load_accounts();
    if let Some(index) = get_selected_index() {
        let i: usize = index.try_into().unwrap_or(0);
        accounts.into_iter().nth(i)
    } else { None }
}

fn save_accounts(accounts: Vec<MCAccount>) -> Result<(), Error> {
    let accounts_json = get_config_dir().join(ACCOUNT_FILE_NAME);
    let json = AccountList {
        accounts,
        selected_index: get_selected_index()
    };

    fs::write(accounts_json, serde_json::to_string_pretty(&json).unwrap())?;
    Ok(())
}

#[tauri::command]
pub fn get_selected_index() -> Option<u32> {
    read_accounts_safe().selected_index
}

#[tauri::command]
pub fn set_selected_index(index: u64) {
    let accounts_json = config_dir().unwrap().join("yamcl").join(ACCOUNT_FILE_NAME);
    let mut json: Value = serde_json::from_str(&fs::read_to_string(&accounts_json).unwrap()).unwrap();
    json["selectedIndex"] = json!(index);
    fs::write(accounts_json, serde_json::to_string_pretty(&json).unwrap()).unwrap();
}