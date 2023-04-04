use crate::structs::region::Region;
use chrono::{DateTime, Local, Utc};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, metadata, File},
    io,
    sync::{mpsc::channel, Arc},
    time::{Duration, Instant, SystemTime},
};

use super::RouterBuilder;

pub(crate) fn mount() -> RouterBuilder {
    // getAppNameをエンドポイントとし、文字列で"rspc Test Project"を返す
    <RouterBuilder>::new()
        // .query("getAppName", |t| t(|ctx, _: ()| "rspc Test Project1111111"))
        .query("playSpeedUpdate", |t| {
            t(|ctx, speed: u8| async move {
                match ctx.sender.send(speed).await {
                    Ok(_) => (),
                    Err(_) => (),
                }
            })
        })
        // .query("initGame", |t| {
        //     t(|ctx, regions: HashMap<i32, Region>| async move {
        //         // match ctx.sender_init_game.send(regions).await {
        //         //     Ok(_) => (),
        //         //     Err(_) => (),
        //         // }
        //     })
        // })
        .query("selectGameId", |t| {
            t(|ctx, game_id: Option<String>| async move {
                match ctx.sender_game_id.send(game_id).await {
                    Ok(_) => (),
                    Err(_) => (),
                }
            })
        })
        .query("refresh", |t| {
            t(|ctx, _: ()| {
                *ctx.refresh_counter.write() += 1;
            })
        })
        .query("getGameData", |t| {
            t(|_, _: ()| {
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
                    .unwrap_or_else(|| {
                        std::env::current_dir().expect("Cannot access the current directory")
                    });
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

                                if let Some(t) = chrono::NaiveDateTime::from_timestamp_opt(
                                    duration.as_secs() as i64,
                                    0,
                                ) {
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
            })
        })
}
