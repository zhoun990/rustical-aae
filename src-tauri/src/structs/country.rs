use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rspc::Type;
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Pool, SqlitePool,
};
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use typeshare::typeshare;

use crate::{game_manager::GAME_MANAGER, getPool};

use super::HandleGameManager;
#[typeshare]
#[derive(Debug, Serialize, Deserialize, Default, sqlx::FromRow, Clone, Type)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub color_primary: String,
    pub color_secondary: String,
    pub capital_city_id: i32,
}
impl Country {
    /// 引数のFnでデフォルトから変更可
    pub async fn new<F: FnOnce(&mut Self) -> ()>(f: F) -> Result<Self> {
        let mut s = Self::default();
        f(&mut s);
        s.add_to_db().await?;
        Ok(s)
    }
    async fn add_to_db(&mut self) -> sqlx::Result<()> {
        let pool = &getPool();
        let mut tx = pool.begin().await?;
        const SQL: &str = "INSERT INTO countries (name, color_primary, color_secondary, capital_city_id) VALUES (?, ?, ?, ?)";
        let result = sqlx::query(SQL)
            .bind(&self.name)
            .bind(&self.color_primary)
            .bind(&self.color_secondary)
            .bind(&self.capital_city_id)
            .execute(&mut tx)
            .await?;
        tx.commit().await?;
        self.id = result.last_insert_rowid() as i32;

        Ok(())
    }
    pub async fn get_from_db() -> sqlx::Result<HashMap<i32, Arc<Mutex<Self>>>> {
        let pool = &getPool();
        let raw = sqlx::query_as::<_, Self>("SELECT * FROM countries")
            .fetch_all(pool)
            .await?;
        let map = raw
            .iter()
            .map(|x| {
                (
                    x.id,
                    Arc::new(Mutex::new(Self {
                        id: x.id,
                        name: x.name.to_string(),
                        color_primary: x.color_primary.to_string(),
                        color_secondary: x.color_secondary.to_string(),
                        capital_city_id: x.capital_city_id,
                    })),
                )
            })
            .collect::<HashMap<i32, Arc<Mutex<Self>>>>();
        Ok(map)
    }
}
#[async_trait]
impl HandleGameManager for Country {
    async fn update_gm(self) -> Result<()> {
        let gm = GAME_MANAGER.lock().await;
        if let Some(st) = gm.countries.get(&self.id) {
            let mut st = st.lock().await;
            self.update_db().await.unwrap();
            *st = self;
            return Ok(());
        };
        Err(anyhow!("citizen is not exists in gm"))
    }
    async fn update_db(&self) -> Result<()> {
        let pool = &getPool();
        let mut tx = pool.begin().await?;
        sqlx::query("UPDATE countries SET name = ?, color_primary = ?, color_secondary = ?, capital_city_id = ? WHERE id = ?")
                .bind(&self.name)
                .bind(&self.color_primary)
                .bind(&self.color_secondary)
                .bind(&self.capital_city_id)
                .bind(&self.id)
                .execute(&mut tx)
                .await?;

        tx.commit().await?;
        Ok(())
    }
}
