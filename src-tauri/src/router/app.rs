use crate::{
    game_manager::{GameData, GAME_MANAGER},
    structs::{citizen::Citizen, city::City, region::Region, HandleGameManager},
};
use anyhow::{anyhow, Error, Result};
use futures::{future, TryStreamExt};

// use async_stream::stream;
use chrono::{DateTime, Local, Utc};
use futures::Stream;
use tokio_stream::{self as stream, wrappers::ReceiverStream, StreamExt};
// use spin_sleep::sleep;
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, metadata, File},
    io,
    sync::{mpsc::channel, Arc},
    time::{Duration, Instant, SystemTime},
};
use tokio::{sync::Mutex, time::sleep};

use super::{Ctx, Igniter, RouterBuilder};
async fn play_speed_update(ctx: Ctx, speed: u8) {
    match ctx.sender.send(speed).await {
        Ok(_) => (),
        Err(_) => (),
    }
}
async fn init_game(ctx: Ctx, regions: HashMap<i32, Region>) {
    match ctx.sender_init_game.send(regions).await {
        Ok(_) => (),
        Err(_) => (),
    }
}

async fn select_game_id(ctx: Ctx, game_id: Option<String>) {
    println!("select game");
    match ctx.sender_game_id.send(game_id).await {
        Ok(_) => (),
        Err(_) => (),
    }
}

fn refresh(ctx: Ctx, _: ()) {
    *ctx.refresh_counter.write() += 1;
}

async fn update<T: HandleGameManager>(_: Ctx, st: T) {
    T::update(st).await.unwrap();
}
// fn game_id_stream(ctx: Ctx, _: ()) -> impl Stream<Item = ()> {
//     let (sender_set_game_id, receiver_set_game_id) = tokio::sync::mpsc::channel(100);
//     let m = ctx.sender_set_game_id.lock().unwrap();
//     m.send(sender_set_game_id).unwrap();
//     ReceiverStream::from(receiver_set_game_id)
// }
async fn game_id(_: Ctx, _: ()) -> String {
    GAME_MANAGER.lock().await.game_id.to_string()
}
// fn game_manager_stream(ctx: Ctx, _: ()) -> impl Stream<Item = ()> {
//     let (sender_set_game_manager, receiver_set_game_manager) = tokio::sync::mpsc::channel(100);
//     let m = ctx.sender_set_game_manager.lock().unwrap();
//     m.send(sender_set_game_manager).unwrap();
//     ReceiverStream::from(receiver_set_game_manager)
// }

fn stream(ctx: Ctx, igniter_type: Igniter) -> impl Stream<Item = Igniter> {
    let (sender, receiver) = tokio::sync::mpsc::channel(100);
    let m = ctx.sender_stream.lock().unwrap();
    m.send(sender).unwrap();
    ReceiverStream::from(receiver).filter(move |v| v == &igniter_type)
}
async fn game_manager(_: Ctx, _: ()) -> GameData {
    let gm = &*GAME_MANAGER.lock().await;
    GameData::from_game_manager(gm).await
}
async fn get_game_data(_: Ctx, _: ()) -> Vec<(String, String)> {
    const DOCUMENTS_DIR: &str = "Documents";
    const APP_DIR: &str = "Webbel";
    const DATABASE_DIR: &str = "AAE/saves";
    // ユーザのホームディレクトリ直下にデータベースのディレクトリを作成する
    // もし、各OSで標準的に使用されるアプリ専用のデータディレクトリに保存したいなら
    // directoriesクレートの`ProjectDirs::data_dir`メソッドなどを使うとよい
    // https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.data_dir
    let home_dir = directories::UserDirs::new()
        .map(|dirs| dirs.home_dir().to_path_buf())
        // ホームディレクトリが取得できないときはカレントディレクトリを使う
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory"));
    let documents_dir = home_dir.join(DOCUMENTS_DIR);
    let app_dir = documents_dir.join(APP_DIR);
    let database_dir = app_dir.join(DATABASE_DIR);
    let paths = fs::read_dir(database_dir).unwrap();

    let mut aae_files = Vec::new();

    for path in paths {
        let entry = &path.unwrap();
        let file_path = entry.path();
        if let Some(extension) = file_path.extension() {
            if extension == "aae" {
                if let Some(file_name) = file_path.file_stem() {
                    // ファイルのメタデータを取得する
                    let metadata = metadata(entry.path()).unwrap();
                    let duration = metadata
                        .modified()
                        .unwrap()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("Failed to convert system time to UNIX timestamp");

                    if let Some(t) =
                        chrono::NaiveDateTime::from_timestamp_opt(duration.as_secs() as i64, 0)
                    {
                        let datetime = DateTime::<Utc>::from_utc(t, Utc);
                        aae_files.push((
                            file_name.to_string_lossy().to_string(),
                            datetime.to_rfc3339(),
                        ));
                    };
                }
            }
        }
    }
    // println!("{:?}", aae_files);
    aae_files
}
pub(crate) fn mount() -> RouterBuilder {
    // getAppNameをエンドポイントとし、文字列で"rspc Test Project"を返す
    <RouterBuilder>::new()
        // .query("getAppName", |t| t(|ctx, _: ()| "rspc Test Project1111111"))
        .query("playSpeedUpdate", |t| t(play_speed_update))
        .query("initGame", |t| t(init_game))
        .query("selectGameId", |t| t(select_game_id))
        .query("refresh", |t| t(refresh))
        .query("updateCitizen", |t| {
            t(|ctx: Ctx, st: Citizen| update(ctx, st))
        })
        .query("updateRegion", |t| {
            t(|ctx: Ctx, st: Region| update(ctx, st))
        })
        .query("updateCity", |t| t(|ctx: Ctx, st: City| update(ctx, st)))
        .query("getGameData", |t| t(get_game_data))
        .mutation("gameId", |t| t(game_id))
        .mutation("gameManager", |t| t(game_manager))
        .subscription("gameId", |t| {
            t(|ctx: Ctx, _: ()| stream(ctx, Igniter::GameId))
        })
        .subscription("gameManager", |t| {
            t(|ctx: Ctx, _: ()| stream(ctx, Igniter::GameData))
        })
}
