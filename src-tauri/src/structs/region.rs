use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Pool, SqlitePool,
};
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use typeshare::typeshare;

use crate::getPool;
#[typeshare]
#[derive(Debug, Serialize, Deserialize, Default, sqlx::FromRow, Clone)]
pub struct Region {
    pub id: i32,
    pub name: String,
    pub product: String,
    pub country_id: Option<i32>,
    pub position_x: i32,
    pub position_y: i32,
}
impl Region {
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
        let result =
            sqlx::query("INSERT INTO regions (id, name, position_x, position_y, product, country_id) VALUES (?, ?, ?, ?, ?, ?)")
               .bind(if self.id != 0 {Some(&self.id)} else {None})
                .bind(&self.name)
                .bind(&self.position_x)
                .bind(&self.position_y)
                .bind(&self.product)
                .bind(&self.country_id)
                .execute(&mut tx)
                .await?;

        tx.commit().await?;
        println!("id b:{}", self.id);
        println!("id a:{}", result.last_insert_rowid());

        self.id = result.last_insert_rowid() as i32;

        Ok(())
    }
    pub async fn get_from_db() -> sqlx::Result<HashMap<i32, Arc<Mutex<Self>>>> {
        let pool = &getPool();
        let raw = sqlx::query_as::<_, Self>("SELECT * FROM regions")
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
                        product: x.product.to_string(),
                        country_id: x.country_id,
                        position_x: x.position_x,
                        position_y: x.position_y,
                    })),
                )
            })
            .collect::<HashMap<i32, Arc<Mutex<Self>>>>();
        Ok(map)
    }
}
