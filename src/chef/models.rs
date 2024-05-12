use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum IngredientType {
    #[default]
    FoodIngredient,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Food {
    pub id: Uuid,
    pub name: String,
    pub brand: String,
    pub cost: f64,
    pub weight: f64,
    pub volume: f64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Portion {
    pub id: Uuid,
    pub ingredient_id: Uuid,
    pub ingredient_type: IngredientType,
    pub recipe_id: Uuid,
    pub amount_w: f64,
    pub amount_v: f64,
    pub amount_u: usize,
}

