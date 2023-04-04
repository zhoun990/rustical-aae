use anyhow::Result;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};
use structs::{citizen::Citizen, city::City, region::Region};
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;
use typeshare::typeshare;

use crate::{db, structs, SQLITE_POOL, TIMESTAMP};
#[derive(Debug, Default)]
pub struct GameManager {
    pub game_id: String,
    pub citizens: HashMap<i32, Arc<Mutex<Citizen>>>,
    pub cities: HashMap<i32, Arc<Mutex<City>>>,
    pub regions: HashMap<i32, Arc<Mutex<Region>>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameData {
    pub game_id: String,
    pub citizens: Vec<(i32, Citizen)>,
    pub cities: Vec<(i32, City)>,
    pub regions: Vec<(i32, Region)>,
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

        GameData {
            game_id: gm.game_id.to_string(),
            citizens,
            cities,
            regions,
            timestamp: *TIMESTAMP.read(),
        }
    }
}
impl GameManager {
    pub async fn from_regions(regions: HashMap<i32, Region>) -> Result<Self> {
        let (pool, game_id) = db::init_db(None).await?;
        *SQLITE_POOL.write() = Some(pool);
        let mut gm = Self::default();
        gm.game_id = game_id;
        for (key, value) in regions.iter() {
            let region = Region::new(|s| {
                //キーの自動生成を行うためマップのidとずれる
                s.id = *key;
                s.name = value.name.to_string();
                s.position_x = value.position_x;
                s.position_y = value.position_y;
                s.product = value.product.to_string();
                s.country_id = value.country_id;
                println!("s:{:?}", s);
            })
            .await
            .expect(&format!("expect:{:?}", value));
            gm.regions.insert(*key, Arc::new(Mutex::new(region)));
            let city = City::new(move |s| {
                s.region_id = *key;
                s.country_id = None
            })
            .await
            .unwrap();
            let city_id = city.id.to_owned();
            gm.cities.insert(0, Arc::new(Mutex::new(city)));
            for i in 0..10 {
                gm.citizens.insert(
                    i,
                    Arc::new(Mutex::new(
                        Citizen::new(|c| {
                            c.staying_city_id = city_id;
                            c.home_city_id = city_id;
                            c.name = "John".to_string() + &i.to_string();
                        })
                        .await
                        .unwrap(),
                    )),
                );
            }
        }
        Ok(gm)
    }
    pub async fn new(id: Option<String>) -> Result<Self> {
        let game_id = if let Some(game_id) = id {
            *SQLITE_POOL.write() = Some(db::init_db(Some(game_id.clone())).await?.0);
            game_id
        } else {
            let (pool, game_id) = db::init_db(id).await?;
            println!("game id:{}", game_id);
            *SQLITE_POOL.write() = Some(pool);
            game_id
        };
        Ok(GameManager {
            game_id: game_id,
            citizens: Citizen::get_from_db().await.unwrap(),
            cities: City::get_from_db().await.unwrap(),
            regions: Region::get_from_db().await.unwrap(),
        })
    }
    pub async fn execute(&mut self, game_speed: u8) -> Result<u32> {
        // println!("{:?}", self.citizens);
        // {
        // let t = TIMESTAMP.lock();
        // for i in 0..10i32 {
        //     // println!("t:{},key:{:?}", *t, *t as i32 * 10 + i);
        //     self.citizens.insert(
        //         *t as i32 * 10 + i,
        //         Arc::new(Mutex::new(Citizen::new(
        //             *t as i32 * 10 + i,
        //             Some(&("John".to_string() + &(*t as i32 * 10 + i).to_string())),
        //         ))),
        //     );
        // }
        // }
        let pf = Instant::now();
        Citizen::execute(&mut self.citizens).await;
        println!(
            "\x1B[1;1H\x1B[2Kduration of execution:{:?}ms",
            pf.elapsed().as_secs_f64() * 1000.0,
        );
        // println!("{:?}", self.citizens[&0]);

        spin_sleep::sleep(Duration::from_millis(50));
        Ok(*TIMESTAMP.read() + 1)
    }
    pub fn wait_duration(game_speed: u8) -> Duration {
        Duration::from_millis(1000 / game_speed.max(1) as u64)
    }
    pub async fn emit_data(&self, app: &AppHandle) {
        app.emit_all("game_data", GameData::from_game_manager(self).await)
            .unwrap();
    }
}
