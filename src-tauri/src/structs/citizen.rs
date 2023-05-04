use self::relation::Gender;
use super::HandleGameManager;
use crate::game_manager::GAME_MANAGER;
use crate::getPool;
use crate::{
    utils::{percentage, random},
    TIMESTAMP,
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt};
use parking_lot::lock_api::MutexGuard;
use parking_lot::{RawMutex, RwLock};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use rspc::Type;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future;
use std::rc::Rc;
use std::thread::spawn;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use typeshare::typeshare;
mod db;
pub mod relation;
mod sub;

// use crate::utils;
#[derive(Debug, Serialize, Deserialize, Default, Clone, Type)]
pub struct Citizen {
    pub id: i32,
    pub name: String,
    pub born_timestamp: u32,
    pub death_timestamp: Option<u32>,
    pub gender: Gender,
    pub job: Option<String>,
    pub staying_city_id: i32,
    pub home_city_id: i32,
    pub country_id: Option<i32>,
    pub relations: HashMap<i32, relation::Relation>,
    pub level: u16,
    pub rank: u16,
    pub exp: u16,
    pub skill_points: u16,
    pub money: u16,
}

impl Citizen {
    pub fn age(&self) -> u32 {
        (*TIMESTAMP.read() - self.born_timestamp) / 24 / 365
    }
    pub async fn birth<F: Fn(&Citizen) -> ()>(&mut self, insert: F) -> Result<Citizen> {
        let age = self.age();
        println!("age:{}", age);
        let child = Citizen::new(|citizen| {
            citizen.name = "child".to_string() + &1.to_string();
            citizen.gender = Gender::random();
        })
        .await?;
        insert(&child);
        Ok(child)
    }
    pub async fn execute(citizens: &mut HashMap<i32, Arc<Mutex<Self>>>) -> Result<()> {
    
        Ok(())
    }
    // pub async fn execute(map: &mut HashMap<i32, Arc<Mutex<Self>>>) -> Result<()> {
    //     let map = RwLock::new(map);
    //     // map.insert(999, Arc::new(Mutex::new(Citizen::new(
    //     //     999,
    //     //     Some(&("John".to_string() + &999.to_string())),
    //     // ))));
    //     let binding = map.read().to_owned();
    //     let iter = binding.iter();
    //     // let r:Vec<_> = iter
    //     let r = futures::stream::iter(binding)
    //         .map(|(key, val)| {
    //             tokio::spawn(async move {
    //                 {
    //                     // let v = &mut *val.lock();
    //                     let mut options: [sub::Option; 3] = [
    //                         sub::Option {
    //                             name: String::from("add_friend"),
    //                             weight: 5,
    //                         },
    //                         sub::Option {
    //                             name: String::from("add_friend_from_community"),
    //                             weight: 100,
    //                         },
    //                         sub::Option {
    //                             name: String::from("update_friend"),
    //                             weight: 100,
    //                         },
    //                     ];
    //                     {
    //                         let m = val.lock().await;
    //                         if m.relations.len() == 0 {
    //                             options[0].weight = 0;
    //                             options[1].weight = 0;
    //                         } else if m.relations.len() < 3 {
    //                             options[2].weight = 20;
    //                         } else {
    //                             options[0].weight = 1;
    //                             options[1].weight = 10;
    //                         }
    //                     }
    //                     let decision = &options[2]; // sub::make_decision(&options);
    //                     match decision.name.as_str() {
    //                         // "add_friend" => {
    //                         //     // let filtered = map
    //                         //     //     .iter()
    //                         //     //     .filter(|(_, target)| {
    //                         //     //         if let Ok(r) = Citizen::lock_fn(
    //                         //     //             val.to_owned(),
    //                         //     //             target.to_owned().to_owned(),
    //                         //     //             |a, b| a.staying_city_id == b.staying_city_id,
    //                         //     //         ) {
    //                         //     //             r
    //                         //     //         } else {
    //                         //     //             false
    //                         //     //         }
    //                         //     //     })
    //                         //     //     .map(|(key, val)| val.clone())
    //                         //     //     .collect::<Vec<_>>();
    //                         //     // let r = Citizen::lock_fn(
    //                         //     //     val.to_owned(),
    //                         //     //     filtered[rand::thread_rng().gen_range(0..filtered.len())].to_owned(),
    //                         //     //     |a, b| {
    //                         //     //         a.relations.insert(
    //                         //     //             b.id,
    //                         //     //             Relation {
    //                         //     //                 id: b.id,
    //                         //     //                 name: b.name.to_owned(),
    //                         //     //                 impression: 0,
    //                         //     //                 relation_type: RelationType::Acquaintance,
    //                         //     //                 last_met_timestamp: TIMESTAMP.lock().to_owned(),
    //                         //     //             },
    //                         //     //         );
    //                         //     //         b.relations.insert(
    //                         //     //             a.id,
    //                         //     //             Relation {
    //                         //     //                 id: a.id,
    //                         //     //                 name: a.name.to_owned(),
    //                         //     //                 impression: 0,
    //                         //     //                 relation_type: RelationType::Acquaintance,
    //                         //     //                 last_met_timestamp: TIMESTAMP.lock().to_owned(),
    //                         //     //             },
    //                         //     //         );
    //                         //     //     },
    //                         //     // );
    //                         // }
    //                         // "add_friend_from_community" => {}
    //                         "update_friend" => {
    //                             // let m = val.lock();
    //                             // let gender = &m.gender;
    //                             if val.lock().await.gender == Gender::Female && percentage(1, 1000)
    //                             {
    //                                 let mut m = val.lock().await;
    //                                 m.birth(|child| {
    //                                     // let m = map.write();
    //                                     // m.insert(child.id, Arc::new(Mutex::new(child.to_owned())));
    //                                 })
    //                                 .await;
    //                             }
    //                             // let mut increase_vec = vec![];
    //                             // let mut decrease_vec = vec![];
    //                             // {
    //                             //     let m = val.lock();
    //                             //     for (key, relation) in &m.relations {
    //                             //         if percentage(
    //                             //             1,
    //                             //             m.relations.len() as i32
    //                             //                 + if relation.relation_type == RelationType::Partner {
    //                             //                     1
    //                             //                 } else {
    //                             //                     2
    //                             //                 },
    //                             //         ) {
    //                             //             if percentage(
    //                             //                 (sub::get_n(relation.impression, -100, 100, -30, 0.5)
    //                             //                     * 100.0) as i32,
    //                             //                 100,
    //                             //             ) {
    //                             //                 increase_vec.push(key.clone());
    //                             //                 // self.increase_relationship(friend);
    //                             //             } else {
    //                             //                 decrease_vec.push(key.clone());
    //                             //                 // self.decrease_relationship(friend);
    //                             //             }
    //                             //         }
    //                             //     }
    //                             // }
    //                             // increase_vec.iter().for_each(|id| {
    //                             //     let b = &map[id];

    //                             //     let r = Citizen::lock_fn(val.to_owned(), b.to_owned(), |a, b| {
    //                             //         let change_variable = sub::get_change_relationship_value(
    //                             //             a.relations[&b.id].impression as f32,
    //                             //         );
    //                             //         if let Some(relation) = a.relations.get_mut(&b.id) {
    //                             //             relation.impression += rand::thread_rng().gen_range(
    //                             //                 1.max(change_variable as i32 - 10)
    //                             //                     ..1.max(change_variable as i32),
    //                             //             );
    //                             //         }
    //                             //         if percentage(1, rand::thread_rng().gen_range(1..=5)) {
    //                             //             let change_variable = sub::get_change_relationship_value(
    //                             //                 b.relations[&a.id].impression as f32,
    //                             //             );
    //                             //             if let Some(relation) = b.relations.get_mut(&a.id) {
    //                             //                 relation.impression += rand::thread_rng().gen_range(
    //                             //                     1.max(change_variable as i32 - 10)
    //                             //                         ..1.max(change_variable as i32),
    //                             //                 );
    //                             //             }
    //                             //         };
    //                             //         // - 恋人になる
    //                             //         // - 妊娠
    //                             //     });
    //                             // });
    //                             // decrease_vec.iter().for_each(|id| {
    //                             //     let b = &map[id];

    //                             //     let r = Citizen::lock_fn(val.to_owned(), b.to_owned(), |a, b| {
    //                             //         let change_variable = sub::get_change_relationship_value(
    //                             //             a.relations[&b.id].impression as f32,
    //                             //         );
    //                             //         if let Some(relation) = a.relations.get_mut(&b.id) {
    //                             //             relation.impression -= rand::thread_rng()
    //                             //                 .gen_range(1..1.max(change_variable as i32));
    //                             //         }
    //                             //         if percentage(1, rand::thread_rng().gen_range(1..=5)) {
    //                             //             let change_variable = sub::get_change_relationship_value(
    //                             //                 b.relations[&a.id].impression as f32,
    //                             //             );
    //                             //             if let Some(relation) = b.relations.get_mut(&a.id) {
    //                             //                 relation.impression -= rand::thread_rng()
    //                             //                     .gen_range(1..1.max(change_variable as i32));
    //                             //             }
    //                             //         };
    //                             //         // - 別れる
    //                             //     });
    //                             // });
    //                         }
    //                         _ => {
    //                             println!("no decision match");
    //                         }
    //                     };
    //                     Ok(val.clone())
    //                 }
    //             })
    //         })
    //         .buffer_unordered(3)
    //         .map(|x| x?)
    //         .try_fold(String::new(), |acc, x| async move {
    //             println!("-res: {:?}", x);
    //             anyhow::Ok(format!("{}:{:?}", acc, x))
    //         });

    //     // .collect::<HashMap<_, _>>();
    //     // Ok(*map.read())
    //     Ok(())
    // }
}
impl Citizen {
    // pub async fn lock(
    //     a: Arc<Mutex<Self>>,
    //     b: Arc<Mutex<Self>>,
    // ) -> (Arc<Mutex<Self>>, Arc<Mutex<Self>>, i32) {
    //     let a_id = { a.lock().await.id };
    //     let b_id = { b.lock().await.id };
    //     // if a_id == b_id {
    //     //     println!("\n!!!Same Mutex id!!!\n")
    //     // };
    //     if a_id < b_id {
    //         (a, b, if a_id == b_id { 2 } else { 0 })
    //     } else {
    //         (b, a, if a_id == b_id { 2 } else { 1 })
    //     }
    // }
    // pub async fn lock_fn<T, F: FnOnce(&mut Citizen, &mut Citizen) -> T>(
    //     a: Arc<Mutex<Self>>,
    //     b: Arc<Mutex<Self>>,
    //     f: F,
    // ) -> Result<T> {
    //     let (a, b, i) = Citizen::lock(a, b).await;
    //     if i == 2 {
    //         return Err(anyhow!("Same Mutex id"));
    //     }
    //     // let c = (*a.lock(), *b.lock(), i);
    //     let a = &mut *a.lock().await;
    //     let b = &mut *b.lock().await;
    //     let r = if i == 0 { f(a, b) } else { f(b, a) };
    //     Ok(r)
    // }
    /// 引数のFnでデフォルトから変更可
    pub async fn new<F: FnOnce(&mut Self) -> ()>(f: F) -> Result<Self> {
        let mut s = Self::default();
        s.gender = Gender::random();
        f(&mut s);
        s.add_to_db().await?;

        Ok(s)
    }
}

#[async_trait]
impl HandleGameManager for Citizen {
    async fn update_gm(self) -> Result<()> {
        let gm = GAME_MANAGER.lock().await;
        if let Some(st) = gm.citizens.get(&self.id) {
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
        sqlx::query(
                "UPDATE citizens SET name = ?, death_timestamp = ?, job = ?, staying_city_id = ?, home_city_id = ?, country_id = ? WHERE id = ?",
            )
            .bind(&self.name)
            .bind(&self.death_timestamp)
            .bind(&self.job)
            .bind(&self.staying_city_id)
            .bind(&self.home_city_id)
            .bind(&self.country_id)
            .bind(&self.id)
            .execute(&mut tx)
            .await?;

        for (target_id, relation) in &self.relations {
            let old_relation = &self.relations[target_id];
            if relation.impression != old_relation.impression
                || relation.relation_type != old_relation.relation_type
            {
                sqlx::query(
                    "UPDATE relationships SET impression = ?, relation_type = ? WHERE id = ?",
                )
                .bind(relation.impression)
                .bind(relation.relation_type.to_string())
                .bind(&relation.id)
                .execute(&mut tx)
                .await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }
}
