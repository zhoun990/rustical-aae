use super::GameManager;
use crate::{
    structs::{citizen::Citizen, city::City, country::Country, item::Item, region::Region},
    TIMESTAMP,
};
use anyhow::Result;
use futures::{StreamExt, TryStreamExt};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rspc::Type;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct GameData {
    pub game_id: String,
    pub citizens: Vec<(i32, Citizen)>,
    pub cities: Vec<(i32, City)>,
    pub regions: Vec<(i32, Region)>,
    pub countries: Vec<(i32, Country)>,
    pub items: Vec<(i32, Item)>,
    pub timestamp: u32,
}

impl GameData {
    pub async fn from_game_manager(gm: &GameManager) -> Self {
        let mut citizens = Vec::new();
        for (id, mutex) in &gm.citizens {
            let citizen = mutex.lock().await.clone();
            citizens.push((id.to_owned(), citizen));
        }
        let mut cities = Vec::new();
        for (id, mutex) in &gm.cities {
            let city = mutex.lock().await.clone();
            cities.push((id.to_owned(), city));
        }
        let mut regions = Vec::new();
        for (id, mutex) in &gm.regions {
            let region = mutex.lock().await.clone();
            regions.push((id.to_owned(), region));
        }
        let mut countries = Vec::new();
        for (id, mutex) in &gm.countries {
            let country = mutex.lock().await.clone();
            countries.push((id.to_owned(), country));
        }
        let mut items = Vec::new();
        for (id, mutex) in &gm.items {
            let item = mutex.lock().await.clone();
            items.push((id.to_owned(), item));
        }
        GameData {
            game_id: gm.game_id.to_string(),
            citizens,
            cities,
            regions,
            items,
            countries,
            timestamp: *TIMESTAMP.read(),
        }
    }
}
