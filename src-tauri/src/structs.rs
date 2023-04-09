pub(crate) mod citizen;
pub(crate) mod city;
pub(crate) mod region;
pub(crate) mod item;

use anyhow::Result;
use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
#[async_trait]
pub trait HandleGameManager {
    // async fn get_from_gm() -> HashMap<i32, Arc<Mutex<Self>>>;
    async fn update(self) -> Result<()>;
}
