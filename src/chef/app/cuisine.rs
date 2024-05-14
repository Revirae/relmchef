use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::chef::models::{Food, FoodPortion, Portion, Recipe};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Cuisine {
    foodlist: HashMap<Uuid, Food>,
    recipelist: HashMap<Uuid, Recipe>,
    portionlist: HashMap<Uuid, Portion>,
}

impl Cuisine {
    pub fn food_list(&self) -> Vec<Food> {
        self.foodlist.clone().into_values().collect()
    }
    pub fn insert_food(&mut self, id: Uuid, food: Food) {
        self.foodlist.insert(id, food);
    }
    pub fn remove_food(&mut self, id: &Uuid) {
        self.foodlist.remove(id);
    }


    pub fn recipe_list(&self) -> Vec<Recipe> {
        self.recipelist.clone().into_values().collect()
    }
    pub fn insert_recipe(&mut self, id: Uuid, recipe: Recipe) {
        self.recipelist.insert(id, recipe);
    }
    pub fn remove_recipe(&mut self, id: &Uuid) {
        self.recipelist.remove(id);
    }

    pub fn portion_list(&self) -> Vec<Portion> {
        self.portionlist.clone().into_values().collect()
    }
    pub fn food_portion_list(&self) -> Vec<FoodPortion> {
        self.portion_list().into_iter().map(|portion| {
            let ingredient = self.foodlist
                .get(&portion.ingredient_id)
                .unwrap()
                .clone();
            let recipe = self.recipelist
                .get(&portion.recipe_id)
                .unwrap()
                .clone();
            FoodPortion {
                inner: portion,
                ingredient, recipe
            }
        }).collect()
    }
    pub fn insert_portion(&mut self, id: Uuid, portion: Portion) {
        self.portionlist.insert(id, portion);
    }
    #[allow(dead_code)]
    pub fn remove_portion(&mut self, id: &Uuid) {
        self.portionlist.remove(id);
    }
}

