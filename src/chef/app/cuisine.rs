use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::chef::models::{Food, FoodPortion, Portion, Product, Recipe};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Cuisine {
    foodmap: HashMap<Uuid, Food>,
    recipemap: HashMap<Uuid, Recipe>,
    portionmap: HashMap<Uuid, Portion>,
    productmap: HashMap<Uuid, Product>,
}

impl Cuisine {
    pub fn food_list(&self) -> Vec<Food> {
        self.foodmap.clone().into_values().collect()
    }
    pub fn insert_food(&mut self, id: Uuid, food: Food) {
        self.foodmap.insert(id, food);
    }
    pub fn remove_food(&mut self, id: &Uuid) {
        self.foodmap.remove(id);
    }

    // pub fn recipe_map(&self) -> &HashMap<Uuid, Recipe> {
        // &self.recipemap
    // }
    pub fn recipe_list(&self) -> Vec<Recipe> {
        self.recipemap.clone().into_values().collect()
    }
    pub fn insert_recipe(&mut self, id: Uuid, recipe: Recipe) {
        self.recipemap.insert(id, recipe);
    }
    pub fn remove_recipe(&mut self, id: &Uuid) {
        self.recipemap.remove(id);
    }

    pub fn portion_list(&self) -> Vec<Portion> {
        self.portionmap.clone().into_values().collect()
    }
    pub fn food_portion_list(&self) -> Vec<FoodPortion> {
        self.portion_list().into_iter().map(|portion| {
            dbg!(portion.clone());
            dbg!(self.recipemap.clone());
            let ingredient = self.foodmap
                .get(&portion.ingredient_id)
                .expect("failed to get ingredient for food portion list")
                .clone();
            let recipe = self.recipemap
                .get(&portion.recipe_id)
                .expect("failed to get recipe for food portion list")
                .clone();
            FoodPortion {
                inner: portion,
                ingredient, recipe
            }
        }).collect()
    }
    pub fn insert_portion(&mut self, id: Uuid, portion: Portion) {
        self.portionmap.insert(id, portion);
    }
    #[allow(dead_code)]
    pub fn remove_portion(&mut self, id: &Uuid) {
        self.portionmap.remove(id);
    }

    #[allow(dead_code)]
    pub fn product_list(&self) -> Vec<Product> {
        self.productmap.clone().into_values().collect()
    }
    pub fn insert_product(&mut self, id: Uuid, product: Product) {
        self.productmap.insert(id, product);
    }
    #[allow(dead_code)]
    pub fn remove_product(&mut self, id: &Uuid) {
        self.productmap.remove(id);
    }
}

