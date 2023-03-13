use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
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
