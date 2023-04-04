// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{
    game_manager::GameManager,
    router::{mount, Ctx},
};
use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use futures::{StreamExt, TryStreamExt};
use once_cell::sync::Lazy;
use parking_lot::{RawMutex, RwLock};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, metadata, File},
    io,
    sync::{mpsc::channel, Arc},
    time::{Duration, Instant, SystemTime},
};
use structs::{citizen::Citizen, city::City, region::Region};
use tauri::{async_runtime::TokioJoinHandle, AppHandle, Manager, State};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};
use typeshare::typeshare;
pub(crate) mod db;
pub(crate) mod game_manager;
mod router;
pub(crate) mod structs;
pub(crate) mod utils;

pub static TIMESTAMP: Lazy<RwLock<u32>> = Lazy::new(|| RwLock::new(0));
pub static SQLITE_POOL: Lazy<RwLock<Option<Pool<Sqlite>>>> = Lazy::new(|| RwLock::new(None));
pub fn getPool() -> Pool<Sqlite> {
    (&*SQLITE_POOL.read()).clone().unwrap()
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// async fn my_custom_command(app_handle: tauri::AppHandle) {
//   let app_dir = app_handle.path_resolver().app_dir();
//   use tauri::GlobalShortcutManager;
//   app_handle.global_shortcut_manager().register("CTRL + U", move || {});
// }

// #[typeshare]
// pub struct PlaySpeedUpdate {
//     pub speed: u8,
// }
// #[tauri::command]
// async fn play_speed_update(sender: State<'_, Sender<u8>>, speed: u8) -> Result<(), &str> {
//     println!("\x1B[2K\x1B[4;1Hchange speed:{}", speed);
//     match sender.send(speed).await {
//         Ok(_) => Ok(()),
//         Err(_) => Err("send error on play_speed_update"),
//     }
// }
// #[tauri::command]
// async fn select_game_id(
//     sender: State<'_, Sender<Option<String>>>,
//     game_id: Option<String>,
// ) -> Result<(), &str> {
//     let Ok(_) = sender.send(game_id).await else{
//         return Err("")
//     };
//     Ok(())
// }
#[tauri::command]
async fn init_game(
    sender: State<'_, Sender<HashMap<i32, Region>>>,
    regions: HashMap<i32, Region>,
) -> Result<(), &str> {
    let Ok(_) = sender.send(regions).await else{
        return Err("")
    };
    Ok(())
}
// #[tauri::command]
// fn refresh(refresh_counter: State<'_, Arc<RwLock<i32>>>) -> Result<(), String> {
//     *refresh_counter.write() += 1;
//     Ok(())
// }
// #[tauri::command]
// fn get_game_data() -> Result<Vec<(String, String)>, String> {
//     const DOCUMENTS_DIR: &str = "Documents";
//     const APP_DIR: &str = "Webbel";
//     const DATABASE_DIR: &str = "AAE/saves";
//     // ユーザのホームディレクトリ直下にデータベースのディレクトリを作成する
//     // もし、各OSで標準的に使用されるアプリ専用のデータディレクトリに保存したいなら
//     // directoriesクレートの`ProjectDirs::data_dir`メソッドなどを使うとよい
//     // https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.data_dir
//     let home_dir = directories::UserDirs::new()
//         .map(|dirs| dirs.home_dir().to_path_buf())
//         // ホームディレクトリが取得できないときはカレントディレクトリを使う
//         .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory"));
//     let documents_dir = home_dir.join(DOCUMENTS_DIR);
//     let app_dir = documents_dir.join(APP_DIR);
//     let database_dir = app_dir.join(DATABASE_DIR);
//     let paths = fs::read_dir(database_dir).unwrap();

//     let mut aae_files = Vec::new();

//     for path in paths {
//         let entry = &path.unwrap();
//         let file_path = entry.path();
//         if let Some(extension) = file_path.extension() {
//             if extension == "aae" {
//                 if let Some(file_name) = file_path.file_stem() {
//                     // ファイルのメタデータを取得する
//                     let metadata = metadata(entry.path()).unwrap();
//                     let duration = metadata
//                         .modified()
//                         .unwrap()
//                         .duration_since(std::time::UNIX_EPOCH)
//                         .expect("Failed to convert system time to UNIX timestamp");

//                     if let Some(t) =
//                         chrono::NaiveDateTime::from_timestamp_opt(duration.as_secs() as i64, 0)
//                     {
//                         let datetime = DateTime::<Utc>::from_utc(t, Utc);
//                         aae_files.push((
//                             file_name.to_string_lossy().to_string(),
//                             datetime.to_rfc3339(),
//                         ));
//                     };
//                 }
//             }
//         }
//     }
//     // println!("{:?}", aae_files);

//     Ok(aae_files)
// }

#[test]
fn _create_svg_with_number_id() {
    let input_string = { r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?><svg></svg>"## };
    let re = regex::Regex::new(r#"id=".+?""#).unwrap(); // 正規表現パターンをコンパイルする
    let mut count = -1;
    let replaced_string = re.replace_all(input_string, |caps: &regex::Captures| {
        // マッチした文字列を処理するクロージャを定義する
        let result = format!("id=\"{}\"", count); // 置き換える文字列を数字のインクリメントに変更する
        count += 1; // カウンターをインクリメント
        result
    });
    fs::write("output.svg", replaced_string.as_ref()).expect("Failed to write file!");
}
#[cfg(test)]
mod test;
// pub async fn test_fn() {
//     let mut game_manager = GameManager::new().await.unwrap();
//     game_manager.execute(1).await;
//     game_manager.execute(1).await;
//     game_manager.execute(1).await;
// }
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1B[2J");
    let (sender, mut receiver) = tokio::sync::mpsc::channel(100);
    let (sender_game_id, mut receiver_game_id) = tokio::sync::mpsc::channel(100);
    let (sender_init_game, mut receiver_init_game) = tokio::sync::mpsc::channel(100);

    let (sender_app, receiver_app) = channel();
    let refresh_counter = Arc::new(RwLock::new(0));
    let r = refresh_counter.clone();
    // サブスレッドの開始
    tokio::spawn(async move {
        let app: &AppHandle = &receiver_app.recv().unwrap();

        let mut game_manager = loop {
            if let Ok(game_id) = receiver_game_id.try_recv() {
                break GameManager::new(game_id).await.unwrap();
            };
            if let Ok(regions) = receiver_init_game.try_recv() {
                break GameManager::from_regions(regions).await.unwrap();
            };
            spin_sleep::sleep(Duration::from_millis(16));
        };
        app.emit_all("set_game_id", game_manager.game_id.to_string())
            .unwrap();
        game_manager.emit_data(app).await;
        let mut play_speed: u8 = 0;
        let mut last_update = Instant::now();
        loop {
            if *r.read() > 1 {
                // spin_sleep::sleep(Duration::from_millis(160));

                *r.write() = 1;
                app.emit_all("set_game_id", game_manager.game_id.to_string())
                    .unwrap();
                game_manager.emit_data(app).await;
            }
            // // メインスレッドからメッセージの受信. 受信するまで処理を待つ
            // if let Ok(val) = receiver.try_recv() {
            //     play_speed = val;
            //     println!("hen na tokoro de new play speed:{}", play_speed);
            // }
            let mut d = if let Some(d) =
                GameManager::wait_duration(play_speed).checked_sub(last_update.elapsed())
            {
                d
            } else {
                Duration::ZERO
            };

            // println!("play_speed:{}", play_speed);

            loop {
                if play_speed == 0 {
                    let val = loop {
                        if let Ok(game_id) = receiver.try_recv() {
                            break game_id;
                        };
                        // if let Ok(r) = receiver_refresh.try_recv() {
                        //     println!("ref");
                        //     if r {
                        //         break 'main;
                        //     }
                        // };
                        if *r.read() > 1 {
                            // spin_sleep::sleep(Duration::from_millis(160));

                            *r.write() = 1;
                            app.emit_all("set_game_id", game_manager.game_id.to_string())
                                .unwrap();
                            game_manager.emit_data(app).await;
                        }
                        spin_sleep::sleep(Duration::from_millis(16));
                    };

                    /*
                     * 0 -> pause
                     * 1 -> 1 hour /sec
                     * 2 -> 2 hour /sec
                     * 3 -> 1 day /sec
                     * 4 -> 2 day /sec
                     * 5 -> 1 month /sec
                     */
                    play_speed = val;

                    last_update = Instant::now();
                    break;
                }
                //n秒(Duration d)待って処理を実行
                if let Ok(val) = receiver.try_recv() {
                    //もし途中で新しい速度になったらやり直し
                    println!("\x1B[2K\x1B[3;1Hnew play speed:{}", val);

                    play_speed = val;
                    last_update = Instant::now();
                    break;
                }
                if d > Duration::ZERO {
                    //100msずつdを減らす
                    let mut wait = Duration::from_millis(100);
                    if d < wait {
                        wait = d
                    }
                    // println!("waiting:{:?}", d);
                    spin_sleep::sleep(wait);
                    d -= wait;
                } else {
                    println!(
                        "\x1B[2K\x1B[2;1H処理を実行(前回:{:?}前),time:{}",
                        last_update.elapsed(),
                        TIMESTAMP.read()
                    );

                    last_update = Instant::now();
                    //処理を実行
                    if let Ok(newTime) = game_manager.execute(play_speed).await {
                        if let Some(mut t) = TIMESTAMP.try_write_for(Duration::from_millis(1000)) {
                            *t = newTime
                        }
                        //  = newTime;
                    }
                    game_manager.emit_data(app).await;

                    break;
                }
            }
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(rspc::integrations::tauri::plugin(mount(), move || Ctx {
            refresh_counter: refresh_counter.clone(),
            sender_init_game: sender_init_game.clone(),
            sender_game_id: sender_game_id.clone(),
            sender: sender.clone(),
        }))
        .invoke_handler(tauri::generate_handler![
            // play_speed_update,
            // select_game_id,
            // get_game_data,
            init_game,
            // refresh
        ])
        .setup(move |app| {
            // let app_handle = app.app_handle();
            // app.manage(app_handle);
            // app.manage(getPool());

            // app.manage(refresh_counter);
            // app.manage(sender_init_game);
            // app.manage(sender_game_id);
            // app.manage(sender);
            sender_app.send(app.app_handle());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
