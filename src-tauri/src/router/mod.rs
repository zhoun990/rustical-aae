use parking_lot::{RawMutex, RwLock};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::structs::region::Region;
mod app;

#[derive(Clone)]
pub struct Ctx {
    pub refresh_counter: Arc<RwLock<i32>>,
    pub sender_init_game: Sender<HashMap<i32, Region>>,
    pub sender_game_id: Sender<Option<String>>,
    pub sender: Sender<u8>
}

pub type Router = rspc::Router<Ctx>;
pub(crate) type RouterBuilder = rspc::RouterBuilder<Ctx>;

pub(crate) fn mount() -> Arc<Router> {
    let config = rspc::Config::new().set_ts_bindings_header("/* eslint-disable */"); // ①

    let config = config.export_ts_bindings(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../src/types/rspc/bindings.ts"), // ②
    );

    <Router>::new()
        .config(config)
        .merge("app.", app::mount()) // ③
        .build()
        .arced()
}
