use serde::{Serialize, Deserialize};


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Food {
    pub name: String,
    pub brand: String,
    pub cost: f64,
    pub weight: f64,
    pub volume: f64,
}
