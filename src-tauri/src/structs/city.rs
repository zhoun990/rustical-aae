use anyhow::{anyhow, Error, Result};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

use crate::getPool;
#[typeshare]
#[derive(Debug, Serialize, Deserialize, Default, sqlx::FromRow, Clone)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub position_x: i32,
    pub position_y: i32,
    pub dev_production: i32,
    pub dev_building: i32,
    pub dev_infrastructure: i32,
    pub exp_dev_production: i32,
    pub exp_dev_building: i32,
    pub exp_dev_infrastructure: i32,
    pub control: i32,
    pub environment: i32,
    pub region_id: i32,
    pub country_id: Option<i32>,
}

impl City {
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
        let result=sqlx::query(
            "INSERT INTO cities
                        (name, position_x, position_y, dev_production, dev_building,
                         dev_infrastructure, exp_dev_production, exp_dev_building,
                         exp_dev_infrastructure, control, environment, region_id, country_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(self.name.as_str())
             .bind(self.position_x)
             .bind(self.position_y)
             .bind(self.dev_production)
             .bind(self.dev_building)
             .bind(self.dev_infrastructure)
             .bind(self.exp_dev_production)
             .bind(self.exp_dev_building)
             .bind(self.exp_dev_infrastructure)
             .bind(self.control)
             .bind(self.environment)
             .bind(self.region_id)
             .bind(self.country_id)
             .execute(&mut tx)
            .await?
             ;

        tx.commit().await?;
        self.id = result.last_insert_rowid() as i32;
        Ok(())
    }
    pub async fn get_from_db() -> sqlx::Result<HashMap<i32, Arc<Mutex<Self>>>> {
        let pool = &getPool();
        let raw = sqlx::query_as::<_, Self>("SELECT * FROM cities")
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
                        position_x: x.position_x,
                        position_y: x.position_y,
                        dev_production: x.dev_production,
                        dev_building: x.dev_building,
                        dev_infrastructure: x.dev_infrastructure,
                        exp_dev_production: x.exp_dev_production,
                        exp_dev_building: x.exp_dev_building,
                        exp_dev_infrastructure: x.exp_dev_infrastructure,
                        control: x.control,
                        environment: x.environment,
                        region_id: x.region_id,
                        country_id: x.country_id,
                    })),
                )
            })
            .collect::<HashMap<i32, Arc<Mutex<Self>>>>();
        Ok(map)
    }
}
