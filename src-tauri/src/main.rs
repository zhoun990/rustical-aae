// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    sync::mpsc::{self, Sender},
    time::{Duration, Instant},
};

use futures::channel::mpsc::Receiver;
use tauri::Manager;
pub(crate) mod agent;
pub(crate) mod db;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn play_speed_update(speed: i32)  {
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // このmain関数はasync fnではないので、asyncな関数を呼ぶのにblock_on関数を使う
    let sqlite_pool = db::init_db().await?;
    let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
    // サブスレッドの開始
    let s = sender.clone();

    tokio::spawn(async move {
        let mut play_speed = 0;
        let mut time = 0;
        let mut last_update = Instant::now();

        loop {
            // メインスレッドからメッセージの受信. 受信するまで処理を待つ
            if let Ok(val) = receiver.try_recv() {
                play_speed = val;
                println!("hen na tokoro de new play speed:{}", play_speed);
            }
            println!("play_speed:{}", play_speed);
            if play_speed == 0 {
                let val = receiver.recv().await;
                if let Some(val) = val {
                    play_speed = val;
                    last_update = Instant::now();
                }
            }
            let mut d =
                Duration::from_millis((1000 / play_speed).min(1000)) - last_update.elapsed();
            loop {
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
                    println!("waiting:{:?}", d,);
                    spin_sleep::sleep(wait);
                    d -= wait;
                } else {
                    println!("処理を実行(前回:{:?}前)", last_update.elapsed());

                    last_update = Instant::now();
                    //処理を実行
                    time += 1;
                    spin_sleep::sleep(Duration::from_millis(50));
                    break;
                }
            }
        }
    });

    // tokio::spawn(async move {
    //     sender.send(1).await;

    //     spin_sleep::sleep(Duration::from_millis(1000));

    //     sender.send(3).await;
    //     // tokio::time::sleep(Duration::from_millis(5000)).await;
    //     // sender.send(1).await;
    //     // tokio::time::sleep(Duration::from_millis(5000)).await;
    //     sender.send(0).await;

    //     //     let mut sum = Duration::ZERO;
    //     //     let mut i = 0;
    //     //     let mut avg = Duration::ZERO;
    //     //     let mut frame = Duration::from_micros(16600);
    //     //     let t_avg = frame.to_owned();
    //     //     println!("\x1B[2J");

    //     //     loop {
    //     //         i += 1;
    //     //         let i_t = Instant::now();
    //     //         spin_sleep::sleep(frame);
    //     //         sum += i_t.elapsed();
    //     //         avg = sum / (i + 1);
    //     //         println!("\x1B[1;1Hduration:{:?},平均:{:?}", i_t.elapsed(), avg);
    //     //         if avg.as_micros().abs_diff(t_avg.as_micros()) < 100 {
    //     //             if avg.as_micros().abs_diff(t_avg.as_micros()) < 20 {
    //     //                 if avg.as_micros().abs_diff(t_avg.as_micros()) < 5{
    //     //                     println!("\x1B[2;12Kgood value!,F:{:?}", frame);
    //     //                 } else {
    //     //                     println!("\x1B[2;12Kalmost good value!,F:{:?}", frame);
    //     //                     if t_avg < avg {
    //     //                         frame -= (avg - t_avg) / 100000
    //     //                     } else {
    //     //                         frame += (t_avg - avg) / 100000
    //     //                     }
    //     //                 }                } else {
    //     //                 println!("\x1B[2;12Knot bad value!,F:{:?}", frame);
    //     //                 if t_avg < avg {
    //     //                     frame -= (avg - t_avg) / 1000
    //     //                 } else {
    //     //                     frame += (t_avg - avg) / 1000
    //     //                 }
    //     //             }
    //     //         } else {
    //     //             println!("\x1B[2;12Kbad value,F:{:?}", frame);
    //     //             if t_avg < avg {
    //     //                 frame -= (avg - t_avg) / 10
    //     //             } else {
    //     //                 frame += (t_avg - avg) / 10
    //     //             }
    //     //         }

    //     //         if i > 999999 {
    //     //             println!("Break!");
    //     //             break;
    //     //         }
    //     //     }

    //     //     // メインスレッドへメッセージの送信
    //     //     // sender2.send("hi".to_string()).unwrap();
    // });
    // let val = String::from("hi");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![play_speed_update])
        .setup(|app| {
            let app_handle = app.app_handle();
            app.manage(app_handle);
            app.manage(sqlite_pool);
            app.manage(sender);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
