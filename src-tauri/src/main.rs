// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;

use plutus::{path, Danmu, DanmuConnect};
use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;
use tokio::{join, spawn};

#[tauri::command]
fn clear_login() -> Result<(), String> {
    plutus::clear_login().expect("");
    Ok(())
}

#[tauri::command]
async fn login() -> Result<(), String> {
    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    let qr = spawn({
        async move {
            match rx.await {
                Ok(qr) => *QR.lock().await = qr,
                Err(_) => {}
            }
        }
    });
    let (_a, _b) = join!(qr, plutus::login(tx));
    Ok(())
}

#[tauri::command]
async fn getqr() -> String {
    QR.lock().await.clone()
}

#[tauri::command(rename_all = "snake_case")]
async fn newdanmu(handle: tauri::AppHandle, room_id: String) -> Result<(), String> {
    if let Ok(window) = tauri::WindowBuilder::new(
        &handle,
        room_id.clone(),
        tauri::WindowUrl::App(format!("{}{}", "/#/danmu/", room_id).parse().unwrap()),
    )
    .build()
    {
        window.set_title(room_id.clone().as_str()).unwrap();
        let (x, y);
        {
            let temp1 = Arc::clone(&POS);
            let mut temp2 = temp1.lock().unwrap();
            (x, y) = temp2.clone();
            temp2.0 += 50;
            temp2.1 += 50;
        }
        window
            .set_position(tauri::LogicalPosition::new(x + 50, y + 50))
            .unwrap();
        Ok(())
    } else {
        Err("Exsist this room".to_string())
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn new_connect(room_id: String) -> Result<(), String> {
    let exist = POOL.lock().await.contains_key(&room_id);
    if exist {
        return Err("error".to_string());
    } else {
        let danmu = DanmuConnect::new(room_id.clone()).await.unwrap();
        let (tx, mut rx) = channel::<Vec<Danmu>>(1000);
        let get = spawn({
            async move {
                while let Some(msg) = rx.recv().await {
                    POOL.lock()
                        .await
                        .entry(room_id.clone())
                        .and_modify(|d| *d = msg)
                        .or_insert(vec![]);
                }
            }
        });
        let (_a, _b) = join!(danmu.start(tx), get);
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn get_danmu(room_id: String) -> Result<Vec<Danmu>, String> {
    if POOL.lock().await.contains_key(&room_id) {
        let connects = Arc::clone(&POOL);
        let re = connects.lock().await.get(&room_id).unwrap().clone();
        Ok(re)
    } else {
        Err("Didn't have this room".to_string())
    }
}

lazy_static! {
    static ref POOL: Arc<Mutex<HashMap<String, Vec<Danmu>>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref QR: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref POS: Arc<std::sync::Mutex<(i32, i32)>> = Arc::new(std::sync::Mutex::new((300, 100)));
}

fn main() {
    path::init().unwrap();
    // init_logger().unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            new_connect,
            get_danmu,
            login,
            getqr,
            newdanmu,
            clear_login
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
