use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum IngredientType {
    #[default]
    FoodIngredient,
}

#[derive(Default, Debug, Clone)]
pub enum IngredientUnity {
    #[default]
    Gram,
    Mililiter,
    Unity,
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
    // pub ingredient_type: IngredientType,
    pub recipe_id: Uuid,
    pub amount_w: f64,
    pub amount_v: f64,
    pub amount_u: f64,
}

#[derive(Debug, Default, Clone)]
pub struct FoodPortion {
    pub inner: Portion,
    pub ingredient: Food,
    pub recipe: Recipe,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Default, Clone)]
pub struct RecipePortion {
    pub inner: Portion,
    pub ingredient: Recipe,
    pub product: Product,
}

impl FoodPortion {
    pub fn set_ingredient(&mut self, ingredient: &Food) {
        self.inner.ingredient_id = ingredient.id;
        self.ingredient = ingredient.clone();
    }

    // pub fn set_recipe(&mut self, recipe: &Recipe) {
    //     self.inner.recipe_id = recipe.id;
    //     self.recipe = recipe.clone();
    // }
}
