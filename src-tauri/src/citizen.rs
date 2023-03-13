use anyhow::{anyhow, Error, Result};
use parking_lot::lock_api::MutexGuard;
use parking_lot::{Mutex, RawMutex};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::thread::spawn;
use std::{sync::Arc, time::Duration};

use crate::TIMESTAMP;
// use crate::citizen::sub::make_decision;
use crate::utils::percentage;
pub(crate) mod sub;

// use crate::utils;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Relation {
    pub id: i32,
    pub name: String,
    pub impression: i32,
    pub relation_type: RelationType,
    pub last_met_timestamp: u128,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Citizen {
    pub id: i32,
    pub name: String,
    pub born_timestamp: u128,
    pub death_timestamp: Option<u128>,
    pub gender: Gender,
    pub job: Option<String>,
    pub staying_city_id: i32,
    pub home_city_id: i32,
    pub country_id: Option<i32>,
    pub relations: HashMap<i32, Relation>,
}
impl Citizen {
    pub fn lock(
        a: Arc<Mutex<Self>>,
        b: Arc<Mutex<Self>>,
    ) -> (Arc<Mutex<Self>>, Arc<Mutex<Self>>, i32) {
        let a_id = { a.lock().id };
        let b_id = { b.lock().id };
        // if a_id == b_id {
        //     println!("\n!!!Same Mutex id!!!\n")
        // };
        if a_id < b_id {
            (a, b, if a_id == b_id { 2 } else { 0 })
        } else {
            (b, a, if a_id == b_id { 2 } else { 1 })
        }
    }
    pub fn lock_fn<T, F: FnOnce(&mut Citizen, &mut Citizen) -> T>(
        a: Arc<Mutex<Self>>,
        b: Arc<Mutex<Self>>,
        f: F,
    ) -> Result<T> {
        let (a, b, i) = Citizen::lock(a, b);
        if i == 2 {
            return Err(anyhow!("Same Mutex id"));
        }
        // let c = (*a.lock(), *b.lock(), i);
        let a = &mut *a.lock();
        let b = &mut *b.lock();
        let r = if i == 0 { f(a, b) } else { f(b, a) };
        Ok(r)
    }
    pub fn new(id: i32, name: Option<&str>) -> Self {
        let mut s = Self::default();
        s.id = id;
        if let Some(name) = name {
            s.name = name.to_owned();
        }
        s
    }
    pub fn day(&mut self) {
        if percentage(5, 100) {
            self.update()
        }
    }
    pub fn update(&mut self) {
        self.job = Some("cleaner".to_string());
    }
    pub fn age(&self) -> i32 {
        if let Some(t) = TIMESTAMP.try_lock_for(Duration::from_millis(1000)) {
            ((*t - self.born_timestamp) / 24 / 365).try_into().unwrap()
        } else {
            0
        }

        // self.job = Some("cleaner".to_string());
    }

    pub fn execute(map: HashMap<i32, Arc<Mutex<Self>>>) -> Result<HashMap<i32, Arc<Mutex<Self>>>> {
        let r = map
            .par_iter()
            .map(|(key, val)| {
                // let v = &mut *val.lock();
                let mut options: [sub::Option; 2] = [
                    sub::Option {
                        name: String::from("add_friend"),
                        weight: 2,
                    },
                    sub::Option {
                        name: String::from("update_friend"),
                        weight: 2,
                    },
                ];
                {
                    let m = val.lock();
                    if m.relations.len() == 0 {
                        options[1].weight = 0
                    } else if m.relations.len() < 3 {
                        options[0].weight += 1;
                    } else {
                        options[0].weight -= 1;
                    }
                }
                let decision = sub::make_decision(&options);
                match decision.name.as_str() {
                    "add_friend" => {
                        let filtered = map
                            .iter()
                            .filter(|(_, target)| {
                                if let Ok(r) = Citizen::lock_fn(
                                    val.to_owned(),
                                    target.to_owned().to_owned(),
                                    |a, b| a.staying_city_id == b.staying_city_id,
                                ) {
                                    r
                                } else {
                                    false
                                }
                            })
                            .map(|(key, val)| val.clone())
                            .collect::<Vec<_>>();
                        let r = Citizen::lock_fn(
                            val.to_owned(),
                            filtered[rand::thread_rng().gen_range(0..filtered.len())].to_owned(),
                            |a, b| {
                                a.relations.insert(
                                    b.id,
                                    Relation {
                                        id: b.id,
                                        name: b.name.to_owned(),
                                        impression: 0,
                                        relation_type: RelationType::Acquaintance,
                                        last_met_timestamp: TIMESTAMP.lock().to_owned(),
                                    },
                                );
                                b.relations.insert(
                                    a.id,
                                    Relation {
                                        id: a.id,
                                        name: a.name.to_owned(),
                                        impression: 0,
                                        relation_type: RelationType::Acquaintance,
                                        last_met_timestamp: TIMESTAMP.lock().to_owned(),
                                    },
                                );
                            },
                        );
                    }
                    "update_friend" => {
                        // update friend relation
                        // self.relationUpdate();
                        // v.search_and_add_friend(map.clone());
                    }
                    _ => {
                        println!("no decision match");
                    }
                }
                (key.clone(), val.clone())
            })
            .collect::<HashMap<_, _>>();
        Ok(r)
    }
}
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}
impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RelationType {
    Child,
    Parent,
    Sibling,
    Partner,
    Acquaintance,
    Clan,
}
impl Default for RelationType {
    fn default() -> Self {
        RelationType::Acquaintance
    }
}

// #[cfg(test)]
// mod test;
// pub(crate) async fn main() {
//     let c = Citizen::default();
//     println!("citizen{:?}", c);
//     assert!(true);
// }
