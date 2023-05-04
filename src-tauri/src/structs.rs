pub(crate) mod citizen;
pub(crate) mod city;
pub(crate) mod country;
pub(crate) mod item;
pub(crate) mod region;
pub(crate) mod battle;

use anyhow::Result;
use async_trait::async_trait;
#[async_trait]
pub trait HandleGameManager {
    // async fn get_from_gm() -> HashMap<i32, Arc<Mutex<Self>>>;
    async fn update_db(&self) -> Result<()>;
    async fn update_gm(self) -> Result<()>;
}
