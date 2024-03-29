#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde_json::json;
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use md5;
use argon2::Argon2;
use hex::encode;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

fn get_appdata_dir() -> String {
    let mut appdata_dir = String::new();
    if cfg!(target_os = "windows") {
        appdata_dir = format!("{}\\{}", std::env::var("APPDATA").unwrap(), "lockbox");
    } else if cfg!(target_os = "linux") {
        appdata_dir = format!("{}{}", std::env::var("HOME").unwrap(), "/.lockbox");
    } else if cfg!(target_os = "macos") {
        appdata_dir = format!("{}{}", std::env::var("HOME").unwrap(), "/Library/Application Support/lockbox");
    }

    appdata_dir
}

fn make_appdata_dir() -> String {
    let appdata_dir = get_appdata_dir();
    if !std::path::Path::new(&appdata_dir).exists() {
        fs::create_dir_all(&appdata_dir).expect("Unable to create appdata directory");
    }
    appdata_dir
}

fn get_database_file() -> String {
    return String::from(get_appdata_dir() + "\\lockbox.lbpwddb");
}

fn check_if_db_exists() -> bool {
    let db_file = get_database_file();
    return Path::new(&db_file).exists()
}

fn sha256(password: &str) -> String {
    let sha256_input = password.as_bytes();
    let mut sha256_hasher = Sha256::new();
    sha256_hasher.update(sha256_input);
    let sha256_output = encode(sha256_hasher.finalize());

    return sha256_output
}

fn ripemd(password: &str) -> String {
    let ripemd_input = password.as_bytes();
    let mut ripemd_hasher = Ripemd160::new();
    ripemd_hasher.update(ripemd_input);
    let ripemd_output = encode(ripemd_hasher.finalize());

    return ripemd_output
}

#[tauri::command]
fn hash_password(password: &str) -> String {
    let sha256_output = sha256(password);
    
    let ripemd_input = sha256_output.clone();
    let ripemd_output = ripemd(&ripemd_input);

    let argon2_instance = Argon2::default();

    let argon2_input = ripemd_output;
    let argon2_salt_non_string = md5::compute(sha256_output.clone().as_bytes());

    let mut argon2_salt = String::new();
    for byte in argon2_salt_non_string.iter() {
        argon2_salt.push_str(&format!("{:02x}", byte));
    }

    let mut output_key_material = [0u8; 32];
    let argon2_output = argon2_instance.hash_password_into(argon2_input.as_bytes(), argon2_salt.as_bytes(), &mut output_key_material);

    let result_string = match argon2_output {
        Ok(_) => {
            let hex_output = output_key_material
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>();
            format!("{}", hex_output)
        }
        Err(err) => format!("{:?}", err),
    };

    return result_string.to_string();
}

#[tauri::command]
fn write_file(key: &str, data: &str) -> String {
    let mc = new_magic_crypt!(hash_password(key), 256);

    let output = mc.encrypt_str_to_base64(data);


    let file = File::create(get_database_file());
    match file {
        Ok(mut file) => {
            file.write(output.as_bytes()).unwrap();
        },
        Err(error) => {
            let _error_string = format!("Error: {}", error);
        }
    }

    return output
}

fn read_file(key: &str) -> String {
    let mc = new_magic_crypt!(hash_password(key), 256);

    let mut file = File::open(get_database_file()).expect("Unable to open the file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let output = mc.decrypt_base64_to_string(&contents).unwrap();

    return output.to_string()
}

fn init_database(key: &str) {
    
    let json_data = json!({
        "accounts": [
            {
                "title": "Github - tim@gmail.com",
                "site": "Github",
                "email": "tim@gmail.com",
                "username": "timtimtim",
                "password": "d0nthackMEpl$",
                "notes": "alt account\n123\n$$$",
                "humanname": "Tim Timothy",
                "yearOfBirth": 2001,
                "monthOfBirth": 12,
                "dayOfBirth": 14,
            }
        ]
    });
    let json_string = serde_json::to_string(&json_data).unwrap();

    write_file(&key, &json_string);
}

#[tauri::command]
fn get_launch_data() -> String {
    return "{}".to_string();
}

#[tauri::command]
fn get_decrypted_data(key_from_user: &str) -> String {
    let key = key_from_user;
    
    if !check_if_db_exists() {
        init_database(&key);
    }
    
    return read_file(key)
}


fn main() {
    make_appdata_dir();
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // greet,
            hash_password,
            get_launch_data,
            get_decrypted_data,
            write_file,
        ])
        
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
