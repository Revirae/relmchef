use serde::{Serialize, Deserialize};


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Food {
    pub name: String,
    pub brand: String,
}
