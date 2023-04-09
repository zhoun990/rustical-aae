use std::{collections::HashMap, default, sync::Arc};

use anyhow::Result;
use rspc::Type;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::getPool;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Type)]
pub struct Item {
    pub count: i32,
    pub id: i32,
    pub name: ItemName,
    pub owner: ItemOwner,
}
#[derive(Debug, Serialize, Deserialize, Default, Clone, sqlx::FromRow)]
pub struct DBItem {
    count: i32,
    id: i32,
    name: String,
    owner_citizen_id: Option<i32>,
    owner_city_id: Option<i32>,
    owner_country_id: Option<i32>,
}

impl Item {
    pub async fn new<F: FnOnce(&mut Self) -> ()>(f: F) -> Result<Self> {
        let mut s = Self::default();
        f(&mut s);
        s.add_to_db().await?;
        Ok(s)
    }
    async fn add_to_db(&mut self) -> sqlx::Result<()> {
        let pool = &getPool();
        let mut tx = pool.begin().await?;
        const SQL: &str = "INSERT INTO items
        (count, name, owner_citizen_id, owner_city_id, owner_country_id) VALUES (?, ?, ?, ?, ?)";
        let result = sqlx::query(SQL)
            .bind(self.count)
            .bind(self.name.to_string())
            .bind(self.owner.to_owner_citizen_id())
            .bind(self.owner.to_owner_city_id())
            .bind(self.owner.to_owner_country_id())
            .execute(&mut tx)
            .await
            .unwrap();

        tx.commit().await.unwrap();
        self.id = result.last_insert_rowid() as i32;
        Ok(())
    }
    pub async fn get_from_db() -> sqlx::Result<HashMap<i32, Arc<Mutex<Self>>>> {
        let pool = &getPool();
        let raw = sqlx::query_as::<_, DBItem>("SELECT * FROM items")
            .fetch_all(pool)
            .await?;
        let map = raw
            .iter()
            .map(|x| {
                (
                    x.id,
                    Arc::new(Mutex::new(Self {
                        id: x.id,
                        name: ItemName::from_string(&x.name),
                        count: x.count,
                        owner: ItemOwner::from_db_item(x),
                    })),
                )
            })
            .collect::<HashMap<i32, Arc<Mutex<Self>>>>();
        Ok(map)
    }
}
#[derive(Debug, Serialize, Deserialize, Default, Clone, Type)]
pub enum ItemOwner {
    Citizen(i32),
    City(i32),
    Country(i32),
    #[default]
    None,
}

impl ItemOwner {
    pub fn to_owner_citizen_id(&self) -> Option<i32> {
        if let Self::Citizen(id) = self {
            Some(*id)
        } else {
            None
        }
    }
    pub fn to_owner_city_id(&self) -> Option<i32> {
        if let Self::City(id) = self {
            Some(*id)
        } else {
            None
        }
    }
    pub fn to_owner_country_id(&self) -> Option<i32> {
        if let Self::Country(id) = self {
            Some(*id)
        } else {
            None
        }
    }
    pub fn from_db_item(item: &DBItem) -> Self {
        if let Some(id) = item.owner_citizen_id {
            Self::Citizen(id)
        } else if let Some(id) = item.owner_city_id {
            Self::City(id)
        } else if let Some(id) = item.owner_country_id {
            Self::Country(id)
        } else {
            Self::None
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Default, Clone, Type)]
pub enum ItemName {
    #[default]
    Food,
    Resource,
    Weapon,
    Money,
}

impl ItemName {
    pub fn to_string(&self) -> String {
        match self {
            ItemName::Food => "Food".to_string(),
            ItemName::Resource => "Resource".to_string(),
            ItemName::Weapon => "Weapon".to_string(),
            ItemName::Money => "Money".to_string(),
        }
    }
    pub fn from_string(s: &str) -> Self {
        match s {
            _ => ItemName::Food,
            "Resource" => ItemName::Resource,
            "Weapon" => ItemName::Weapon,
            "Money" => ItemName::Money,
        }
    }
}
