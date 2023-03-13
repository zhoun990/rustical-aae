// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use citizen::Citizen;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RawMutex};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    time::{Duration, Instant},
};

use tauri::{AppHandle, Manager, State};
pub static TIMESTAMP: Lazy<Mutex<u128>> = Lazy::new(|| Mutex::new(0));

pub(crate) mod citizen;
pub(crate) mod city;
pub(crate) mod db;
pub(crate) mod utils;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// async fn my_custom_command(app_handle: tauri::AppHandle) {
//   let app_dir = app_handle.path_resolver().app_dir();
//   use tauri::GlobalShortcutManager;
//   app_handle.global_shortcut_manager().register("CTRL + U", move || {});
// }
#[tauri::command]
async fn play_speed_update(
    sender: State<'_, tokio::sync::mpsc::Sender<u8>>,
    speed: u8,
) -> Result<(), &str> {
    println!("change speed:{}", speed);
    match sender.send(speed).await {
        Ok(_) => Ok(()),
        Err(_) => Err("send error on play_speed_update"),
    }
}
#[derive(Debug, Default)]
struct GameManager {
    pub citizens: HashMap<i32, Arc<Mutex<Citizen>>>,
    pub cities: HashMap<i32, city::City>,
}
impl GameManager {
    pub fn new() -> Self {
        let mut s = Self::default();
        for i in 0..10000 {
            s.citizens.insert(
                i,
                Arc::new(Mutex::new(Citizen::new(
                    i,
                    Some(&("John".to_string() + &i.to_string())),
                ))),
            );
        }
        // s.citizens
        //     .insert(1, Arc::new(Mutex::new(Citizen::new(1, Some("John")))));
        // s.citizens
        //     .insert(2, Arc::new(Mutex::new(Citizen::new(2, Some("Maria")))));
        // s.citizens
        //     .insert(3, Arc::new(Mutex::new(Citizen::new(3, Some("Pochi")))));
        s
    }
    pub fn execute(&mut self, game_speed: u8) -> Result<u128> {
        // println!("{:?}", self.citizens);
        let pf = Instant::now();
        self.citizens = Citizen::execute(self.citizens.clone()).unwrap();
        println!(
            "duration of execution:{:?}ms",
            pf.elapsed().as_secs_f64() * 1000.0
        );
        println!("{:?}", self.citizens[&0]);

        spin_sleep::sleep(Duration::from_millis(50));
        Ok(*TIMESTAMP.lock() + 1)
    }
    pub fn wait_duration(game_speed: u8) -> Duration {
        Duration::from_millis(1000 / game_speed.max(1) as u64)
    }
}

#[cfg(test)]
mod test;
pub async fn test_fn() {
    let mut game_manager = GameManager::new();
    game_manager.execute(1);
    game_manager.execute(1);
    game_manager.execute(1);
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sqlite_pool = db::init_db().await?;
    let (sender, mut receiver) = tokio::sync::mpsc::channel(100);
    // サブスレッドの開始
    tokio::spawn(async move {
        let mut game_manager = GameManager::new();
        let mut play_speed: u8 = 0;
        let mut last_update = Instant::now();
        loop {
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

            println!("play_speed:{}", play_speed);

            loop {
                if play_speed == 0 {
                    let val = receiver.recv().await;
                    if let Some(val) = val {
                        /*
                         * 0 -> pause
                         * 1 -> 1 hour /sec
                         * 2 -> 2 hour /sec
                         * 3 -> 1 day /sec
                         * 4 -> 2 day /sec
                         * 5 -> 1 month /sec
                         */
                        play_speed = val;
                    }
                    last_update = Instant::now();
                    break;
                }
                //n秒(Duration d)待って処理を実行
                if let Ok(val) = receiver.try_recv() {
                    //もし途中で新しい速度になったらやり直し
                    println!("new play speed:{}", val);

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
                        "処理を実行(前回:{:?}前),time:{}",
                        last_update.elapsed(),
                        TIMESTAMP.try_lock().expect("lock faild")
                    );

                    last_update = Instant::now();
                    //処理を実行
                    if let Ok(newTime) = game_manager.execute(play_speed) {
                        if let Some(mut t) = TIMESTAMP.try_lock_for(Duration::from_millis(1000)) {
                            *t = newTime
                        }
                        //  = newTime;
                    }

                    break;
                }
            }
        }
    });
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![play_speed_update])
        .setup(|app| {
            // let app_handle = app.app_handle();
            // app.manage(app_handle);
            app.manage(sqlite_pool);
            app.manage(sender);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
