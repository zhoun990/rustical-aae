// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{
    game_manager::{GameManager, GAME_MANAGER},
    router::{mount, Ctx, Igniter},
};
use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use futures::{stream::FuturesUnordered, StreamExt, TryStreamExt};
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
struct RouterSender<T> {
    senders: Vec<Sender<T>>,
    receiver: std::sync::mpsc::Receiver<Sender<T>>,
}

impl<T: Clone + Send + 'static> RouterSender<T> {
    fn new(receiver: std::sync::mpsc::Receiver<Sender<T>>) -> Self {
        Self {
            senders: vec![loop {
                if let Ok(sender) = receiver.recv() {
                    break sender;
                }
            }],
            receiver,
        }
    }
    fn check_new_sender(&mut self) {
        if let Ok(val) = self.receiver.try_recv() {
            self.senders.push(val);
            self.check_new_sender();
        }
    }
    async fn send(&mut self, value: T) -> &mut Self {
        self.check_new_sender();
        let senders = self.senders.clone();

        tokio::spawn(async move {
            let send_tasks: FuturesUnordered<_> = senders
                .iter()
                .map(|sender| sender.send(value.clone()))
                .collect();
            send_tasks.collect::<Vec<_>>().await;

            //    sender.iter().for_each(||{}).send(value).await
        })
        .await
        .unwrap();
        self
    }
    async fn recreate(&mut self, value: T) -> &mut Self {
        self.senders = vec![loop {
            if let Ok(sender) = self.receiver.recv() {
                break sender;
            }
        }];
        self.send(value).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1B[2J");
    let (sender, mut receiver) = tokio::sync::mpsc::channel(100);
    let (sender_game_id, mut receiver_game_id) = tokio::sync::mpsc::channel(100);
    // let (sender_set_game_id, receiver_set_game_id) = channel();
    // let (sender_set_game_manager, receiver_set_game_manager) = channel();
    let (sender_stream, receiver_stream) = channel();
    let (sender_init_game, mut receiver_init_game) = tokio::sync::mpsc::channel(100);
    let (sender_app, receiver_app) = channel();
    let refresh_counter = Arc::new(RwLock::new(0));
    let r = refresh_counter.clone();

    // tokio::spawn(async move {
    //     let mut sender_set_game_id = RouterSender::new(receiver_set_game_id).await;

    //     for i in 0..50 {
    //         println!("Client subscribed to 'pings'",);
    //         tokio::time::sleep(Duration::from_secs(2)).await;
    //         sender_set_game_id.send(i.to_string()).await;
    //     }
    // });
    // サブスレッドの開始
    tokio::spawn(async move {
        // let app: &AppHandle = &receiver_app.recv().unwrap();

        loop {
            if let Ok(game_id) = receiver_game_id.try_recv() {
                break GameManager::new(game_id).await.unwrap();
            };
            if let Ok(regions) = receiver_init_game.try_recv() {
                break GameManager::from_regions(regions).await.unwrap();
            };
            spin_sleep::sleep(Duration::from_millis(16));
        }
        let mut sender_stream = RouterSender::new(receiver_stream);
        sender_stream.send(Igniter::GameId).await;
        sender_stream.send(Igniter::GameData).await;
        let mut play_speed: u8 = 0;
        let mut last_update = Instant::now();
        loop {
            if *r.read() > 1 {
                *r.write() = 1;
                sender_stream
                    .recreate(Igniter::GameId)
                    .await
                    .send(Igniter::GameData)
                    .await;
                play_speed = 0;
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

            'outer: loop {
                if play_speed == 0 {
                    let val = loop {
                        if let Ok(game_speed) = receiver.try_recv() {
                            break game_speed;
                        };
                        // if let Ok(r) = receiver_refresh.try_recv() {
                        //     println!("ref");
                        //     if r {
                        //         break 'main;
                        //     }
                        // };
                        if *r.read() > 1 {
                            break 'outer;
                            // *r.write() = 1;
                            // sender_stream.recreate(()).await;
                            // sender_set_game_manager.recreate(()).await;
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
                    if let Ok(newTime) = GameManager::execute(play_speed).await {
                        if let Some(mut t) = TIMESTAMP.try_write_for(Duration::from_millis(1000)) {
                            *t = newTime
                        }
                        //  = newTime;
                    }
                    // GameManager::emit_data(app).await;
                    sender_stream.send(Igniter::GameData).await;
                    break;
                }
            }
        }
    });
    // let sender_set_game_id = Arc::new(std::sync::Mutex::new(sender_set_game_id));
    // let sender_set_game_manager = Arc::new(std::sync::Mutex::new(sender_set_game_manager));
    let sender_stream = Arc::new(std::sync::Mutex::new(sender_stream));
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(rspc::integrations::tauri::plugin(mount(), move || Ctx {
            refresh_counter: refresh_counter.clone(),
            sender_init_game: sender_init_game.clone(),
            sender_game_id: sender_game_id.clone(),
            // sender_set_game_id: sender_set_game_id.clone(),
            // sender_set_game_manager: sender_set_game_manager.clone(),
            sender_stream: sender_stream.clone(),
            sender: sender.clone(),
        }))
        .invoke_handler(tauri::generate_handler![
            // play_speed_update,
            // select_game_id,
            // get_game_data,
            // init_game,
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
