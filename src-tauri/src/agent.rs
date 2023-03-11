use parking_lot::Mutex;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::thread::spawn;

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
fn thread((lower_id, higher_id, i): (&Mutex<Agent>, &Mutex<Agent>, i32), t: i32) {
    for n in 0..10 {
        // println!("{},hoi,{}", t, n);
        let mut lower_agent = lower_id.lock();
        // println!("{},poi,{}", t, n);

        let mut higher_agent = higher_id.lock();
        // let mut agent = agent.lock().unwrap();
        // {
        //     let mut agent = agent.lock().unwrap();
        //     agent.move_randomly();
        // }
        // for other_agent in agents {
        // let (lower_id, higher_id) = {
        //     let a1 = agent.lock().unwrap();
        //     let a2 = another_agent.lock().unwrap();
        //     if a1.id < a2.id {
        //         drop(a1);
        //         drop(a2);
        //         (agent, another_agent)
        //     } else {
        //         drop(a1);
        //         drop(a2);
        //         (another_agent, agent)
        //     }
        // };

        // !std::ptr::eq(&another_agent, &agent)
        if lower_agent.name != higher_agent.name {
            lower_agent.add_relationship(&*higher_agent.name, 0.1);

            higher_agent.add_relationship(&*lower_agent.name, 0.1)
        }
        drop(lower_agent);
        drop(higher_agent);
        // println!("{},hey,{}", t, n);

        // }
    }
}
pub(crate) async fn main() {
    const N_AGENTS: usize = 100;

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
        let random_index = rng.gen_range(0..agents.len());
        // let some = agents[random_index].clone();
        // let some = some.lock();
        let ids = {
            // let another = agents[random_index].clone();
            // let agent = agents[].clone();
            if id < random_index {
                (id, random_index, 0)
            } else {
                (random_index, id, 1)
            }
        };
        // `choose` メソッドを使用して `Vec` からランダムに要素を取得する
        // let random_item = agents[t + 1].clone();

        let handle = tokio::spawn(async move {
            println!("s:{}", t);
            for _ in 0..10 {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                // {
                //     let mut agent = agents[if ids.2 == 0 { ids.0 } else { ids.1 }].lock();
                //     agent.move_randomly();
                // }

                let mut lower_agent = agents[ids.0].lock();

                let mut higher_agent = agents[ids.1].lock();
                if lower_agent.name != higher_agent.name {
                    lower_agent.add_relationship(&*higher_agent.name, 0.1);
                    higher_agent.add_relationship(&*lower_agent.name, 0.1)
                }
                // drop(lower_agent);
                // drop(higher_agent);
            }
            println!("e:{}", t);
        });

        println!("ron,{}", t);
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
    println!("pen");

    for handle in handles {
        println!("ron");
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
}
