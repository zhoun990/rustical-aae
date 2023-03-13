use parking_lot::lock_api::MutexGuard;
use parking_lot::{Mutex, RawMutex};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::thread::spawn;
use std::{sync::Arc, time::Duration};

use crate::utils::percentage;
// use crate::utils;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Citizen {
    pub id: i32,
    pub name: String,
    pub born_timestamp: i64,
    pub death_timestamp: Option<i64>,
    pub gender: Gender,
    pub job: Option<String>,
    pub staying_city_id: i32,
    pub home_city_id: i32,
    pub country_id: Option<i32>,
}
impl Citizen {
    pub fn day() {
        if percentage(5, 100) {}
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
#[test]
fn citizen_day_test() {
    let c = Citizen::default();
    println!("citizen{:?}", c);
    assert!(true);
}
#[derive(Clone, Debug, Default)]
struct Agent {
    id: i32,
    name: String,
    x: i32,
    y: i32,
    relationships: Arc<Mutex<Vec<(String, f64)>>>,
}

impl Agent {
    fn new(id: i32, name: &str, x: i32, y: i32) -> Self {
        Self {
            id,
            name: name.to_string(),
            x,
            y,
            relationships: Arc::new(Mutex::new(vec![])),
        }
    }

    fn move_randomly(&mut self) {
        let mut rng = thread_rng();
        let (new_x, new_y) = loop {
            let x = self.x + rng.gen_range(-1..=1);
            let y = self.y + rng.gen_range(-1..=1);
            if x != self.x || y != self.y {
                break (x, y);
            }
        };
        self.x = new_x;
        self.y = new_y;
    }

    fn add_relationship(&mut self, agent_name: &str, value: f64) {
        let mut relationships = self.relationships.lock();
        relationships.push((agent_name.to_string(), value));
    }
}

#[cfg(test)]
mod test;
fn factorial(num: u128) -> u128 {
    if num == 0 || num == 1 {
        1
    } else {
        num * factorial(num - 1)
    }
}
pub(crate) async fn main() {

    // for _ in 0..1000 {
        const N_AGENTS: usize = 1000;

        let mut agents = Vec::new();
        for i in 0..N_AGENTS {
            let name = format!("Agent {}", i + 1);
            let agent: Arc<Mutex<Agent>> = Arc::new(Mutex::new(Agent::new(i as i32, &name, 0, 0)));
            agents.push(agent.clone());
        }

        let mut handles = Vec::new();
        let mut t = 0;
        for id in 0..N_AGENTS {
            // let agent_name = agent.lock().unwrap().name.clone();
            let agents = agents.clone();
            let mut rng = rand::thread_rng();
            let mut random_index = rng.gen_range(0..agents.len());
            // let some = agents[random_index].clone();
            // let some = some.lock();
            let ids = {
                loop {
                    if id == random_index {
                        random_index = rand::thread_rng().gen_range(0..agents.len())
                    } else {
                        break;
                    }
                }

                // let another = agents[random_index].clone();
                // let agent = agents[].clone();
                if id < random_index {
                    (id, random_index, 0)
                } else {
                    (random_index, id, 1)
                }
            };
            println!("ids:{:?}", ids);
            // `choose` メソッドを使用して `Vec` からランダムに要素を取得する
            // let random_item = agents[t + 1].clone();

            let handle = tokio::spawn(async move {
                // println!("s:{}", t);

                for _ in 0..10 {
                    // {
                    //     let mut agent = agents[if ids.2 == 0 { ids.0 } else { ids.1 }].lock();
                    //     agent.move_randomly();
                    // }
                    let mut lower_agent: Option<MutexGuard<RawMutex, Agent>> = None;
                    let mut higher_agent: Option<MutexGuard<RawMutex, Agent>> = None;
                    // if let Some(agent) = agents[ids.0].try_lock_for(Duration::from_secs(1)) {
                    //     agent
                    // } else {
                    //     spin_sleep::sleep(Duration::from_secs(1));
                    //     agents[ids.0]
                    //         .try_lock_for(Duration::from_secs(1))
                    //         .expect("lower_agent timeout")
                    // };
                    // let mut higher_agent =
                    //     if let Some(agent) = agents[ids.1].try_lock_for(Duration::from_secs(1)) {
                    //         agent
                    //     } else {
                    //         spin_sleep::sleep(Duration::from_secs(1));
                    //         agents[ids.1]
                    //             .try_lock_for(Duration::from_secs(1))
                    //             .expect("higher_agent timeout")
                    //     };
                    // tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    loop {
                        if let Some(agent) = agents[ids.0].try_lock_for(Duration::from_secs(1)) {
                            lower_agent = Some(agent);
                            break;
                        } else {
                            let wait = rand::thread_rng().gen_range(1000..3000);
                            println!(
                            "{},lower_agent:Mutexの取得に失敗しました。{:?}秒後にリトライします。",
                            t,
                            wait as f32 / 1000 as f32
                        );
                            spin_sleep::sleep(Duration::from_millis(wait));
                        };
                    }
                    loop {
                        if let Some(agent) = agents[ids.1].try_lock_for(Duration::from_secs(1)) {
                            higher_agent = Some(agent);
                            break;
                        } else {
                            let wait = rand::thread_rng().gen_range(1000..3000);
                            println!(
                            "{},lower_agent:Mutexの取得に失敗しました。{:?}秒後にリトライします。",
                            t,
                            wait as f32 / 1000 as f32
                        );
                            spin_sleep::sleep(Duration::from_millis(wait));
                        };
                    }
                    // spin_sleep::sleep(Duration::from_millis(100));

                    let mut lower_agent = lower_agent.unwrap();
                    let mut higher_agent = higher_agent.unwrap();
                    // let mut higher_agent = agents[ids.1]
                    //     .try_lock_for(Duration::from_secs(1))
                    //     .expect("higher_agent timeout");

                    if lower_agent.name != higher_agent.name {
                        lower_agent.add_relationship(&*higher_agent.name, 0.1);
                        higher_agent.add_relationship(&*lower_agent.name, 0.1)
                    }
                    // drop(lower_agent);
                    // drop(higher_agent);
                }
                // println!("e:{}", t);
            });
            t += 1;
            handles.push(handle);

            // let (a, b, i) = {
            //     let agent = agent.clone();
            //     let random_item = random_item.clone();
            //     let a1 = agent.lock();
            //     let a2 = random_item.lock();
            //     if a1.id < a2.id {
            //         drop(a1);
            //         drop(a2);
            //         (agent, random_item, 0 as i32)
            //     } else {
            //         drop(a1);
            //         drop(a2);
            //         (random_item, agent, 1 as i32)
            //     }
            // };
        }
        // tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;

        for handle in handles {
            // println!("ron");
            handle.await.unwrap();
        }

        // for agent in agents {
        let agent = agents[0].lock();
        println!(
            "{} is at ({}, {}), relationships: {:?}",
            agent.name,
            agent.x,
            agent.y,
            agent.relationships.lock()
        );
    // }
    // }
}
