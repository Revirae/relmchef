use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Food {
    pub id: Uuid,
    pub name: String,
    pub brand: String,
    pub cost: f64,
    pub weight: f64,
    pub volume: f64,
}

// impl Default for Food {
//     fn default() -> Self {
//         Self {
//             id: Uuid::new_v4(),
//             ..Default::default()
//         }
//     }
// }
