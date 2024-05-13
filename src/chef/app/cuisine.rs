use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::chef::models::{Food, Portion, Recipe};

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
    pub fn insert_portion(&mut self, id: Uuid, portion: Portion) {
        self.portionlist.insert(id, portion);
    }
    pub fn remove_portion(&mut self, id: &Uuid) {
        self.portionlist.remove(id);
    }
}

