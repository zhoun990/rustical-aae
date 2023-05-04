use crate::{
    db,
    structs::{
        battle::BattleState,
        citizen::Citizen,
        city::City,
        country::Country,
        item::{Item, ItemName, ItemOwner},
        region::Region,
        HandleGameManager,
    },
    utils::{percentage, random},
    SQLITE_POOL, TIMESTAMP,
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
pub mod game_data;

pub static GAME_MANAGER: Lazy<Mutex<GameManager>> =
    Lazy::new(|| Mutex::new(GameManager::default()));

#[derive(Debug, Default)]
pub struct GameManager {
    pub game_id: String,
    pub citizens: HashMap<i32, Arc<Mutex<Citizen>>>,
    pub cities: HashMap<i32, Arc<Mutex<City>>>,
    pub regions: HashMap<i32, Arc<Mutex<Region>>>,
    pub countries: HashMap<i32, Arc<Mutex<Country>>>,
    pub items: HashMap<i32, Arc<Mutex<Item>>>,
    pub battles: HashMap<i32, Arc<Mutex<BattleState>>>,
}
impl GameManager {
    pub async fn from_regions(regions: HashMap<i32, Region>) -> Result<()> {
        let (pool, game_id) = db::init_db(None).await.unwrap();
        *SQLITE_POOL.write() = Some(pool);
        let mut gm = Self::default();
        gm.game_id = game_id;

        for (key, value) in regions.iter() {
            let region = Region::new(|s| {
                s.id = key.to_owned();
                s.name = value.name.to_string();
                s.position_x = value.position_x;
                s.position_y = value.position_y;
                s.product = value.product.to_string();
                // s.country_id = value.country_id;
            })
            .await
            .expect(&format!("expect:{:?}", value));

            gm.regions
                .insert(key.to_owned(), Arc::new(Mutex::new(region)));

            let mut city = City::new(move |s| {
                s.region_id = key.to_owned();
                s.country_id = None;
                s.name = "City".to_string();
            })
            .await
            .unwrap();

            let city_id = city.id.to_owned();
            for _ in 1..10 {
                let item = Item::new(|item| {
                    item.count = 10;
                    item.owner = ItemOwner::City(city_id);
                })
                .await
                .unwrap();
                gm.items.insert(item.id, Arc::new(Mutex::new(item)));
            }
            let country = Country::new(|s| {
                s.id = key.to_owned();
                s.name = "Country".to_string();
                s.capital_city_id = city_id;
            })
            .await
            .expect(&format!("expect:{:?}", value));

            city.country_id = Some(country.id);
            city.update_db().await.unwrap();
            gm.cities.insert(city.id, Arc::new(Mutex::new(city)));
            gm.countries
                .insert(key.to_owned(), Arc::new(Mutex::new(country)));

            for i in 0..10 {
                let citizen = Citizen::new(|c| {
                    c.staying_city_id = city_id;
                    c.home_city_id = city_id;
                    c.name = "John".to_string() + &i.to_string();
                    c.rank = random(1, 10) as u16;
                })
                .await
                .unwrap();
                for _ in 1..10 {
                    let item = Item::new(|item| {
                        item.count = 10;
                        item.owner = ItemOwner::Citizen(citizen.id);
                        item.name = ItemName::Money;
                    })
                    .await
                    .unwrap();
                    gm.items.insert(item.id, Arc::new(Mutex::new(item)));
                }
                gm.citizens
                    .insert(citizen.id, Arc::new(Mutex::new(citizen)));
            }
        }
        *GAME_MANAGER.lock().await = gm;
        // let cities = {
        //     let gm = GAME_MANAGER.lock().await;
        //     gm.cities.clone()
        // };
        // for (key, value) in cities.iter() {
        //     let city = value.lock().await;
        //     let country = Country::new(|s| {
        //         s.id = key.to_owned();
        //         s.name = "Country".to_string();
        //         s.capital_city_id = city.id;
        //     })
        //     .await
        //     .expect(&format!("expect:{:?}", value));
        // }

        Ok(())
    }
    pub async fn new(id: Option<String>) -> Result<()> {
        let game_id = if let Some(game_id) = id {
            *SQLITE_POOL.write() = Some(db::init_db(Some(game_id.clone())).await?.0);
            game_id
        } else {
            let (pool, game_id) = db::init_db(id).await?;
            println!("game id:{}", game_id);
            *SQLITE_POOL.write() = Some(pool);
            game_id
        };
        let mut gm = GameManager {
            game_id,
            citizens: Citizen::get_from_db().await.unwrap(),
            cities: City::get_from_db().await.unwrap(),
            regions: Region::get_from_db().await.unwrap(),
            countries: Country::get_from_db().await.unwrap(),
            items: Item::get_from_db().await.unwrap(),
            battles: HashMap::new(),
        };
        gm.battles
            .insert(0, Arc::new(Mutex::new(BattleState::template(0, 1))));
        *GAME_MANAGER.lock().await = gm;
        Ok(())
    }
    pub async fn execute(game_speed: u8) -> Result<u32> {
        let gm = &mut *GAME_MANAGER.lock().await;
        let pf = Instant::now();
        let citizens = RwLock::new(&mut gm.citizens);
        let binding = citizens.read().to_owned();
        let r = futures::stream::iter(binding)
            .map(|(key, val)| {
                let a = tokio::spawn(async move {
                    {
                        let mut m = val.lock().await;
                        m.money += m.rank * 10;
                        m.money -= m.rank * 7;
                        if percentage(1, 100) {
                            m.money -= m.money / 2;
                        }
                        // if *TIMESTAMP.read() % 30 == 0 {
                        //     let city = &gm.cities.get(&m.staying_city_id);
                        // }
                        Ok(val.clone())
                    }
                });
                a
            })
            .buffer_unordered(3)
            .map(|x| x?)
            .try_fold(String::new(), |acc, x| async move {
                // println!("-res: {:?}", x);
                anyhow::Ok(format!("{}:{:?}", acc, x))
            });
        let l = r.await;
        println!(
            "\x1B[1;1H\x1B[2Kduration of execution:{:?}ms",
            pf.elapsed().as_secs_f64() * 1000.0,
        );
        spin_sleep::sleep(Duration::from_millis(50));
        Ok(*TIMESTAMP.read() + 1)
    }
    pub fn wait_duration(game_speed: u8) -> Duration {
        Duration::from_millis(1000 / game_speed.max(1) as u64)
    }
}
