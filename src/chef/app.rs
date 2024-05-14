// #![allow(unused)]
mod cuisine;

// use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use uuid::Uuid;

use crate::chef::components::recipe_page::{RecipePageMessage, RecipePageState};
use crate::chef::components::header;

use relm4::gtk;
use gtk::prelude::{
    GtkWindowExt, OrientableExt, ApplicationExt
};
use relm4::{
    SimpleComponent,
    Component,
    ComponentParts,
    ComponentSender,
    Controller,
    ComponentController
};
use serde::{Deserialize, Serialize};

use self::cuisine::Cuisine;

use super::components::food_page::{FoodPageCommand, FoodPageMessage, FoodPageModel, FoodPageState};
use super::components::header::HeaderModel;
use super::components::recipe_page::{RecipePageCommand, RecipePageModel};
use super::models::{Food, Portion, Recipe};

#[derive(Default, Debug)]
pub enum AppMode {
    #[default]
    FoodInventory,
    Recipes,
}

#[derive(Default)]
pub struct AppState {
    database_path: String,
    page: String,
}

impl AppState {
    pub fn new(database_path: String) -> Self {
        AppState {
            database_path,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppData {
    cuisine: Cuisine,
}

impl AppData {
    fn from_file(path: String) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader)?;
        Ok(data)
    }
    fn to_file(&self, path: String) -> std::io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        dbg!(self);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub enum AppCommand {
    #[default] 
    NoCommand,
    CloseRequest,
    LoadDatabase,
    PersistDatabase,
    SetMode(AppMode),
    AddFood(Food),
    RemoveFood(Uuid),
    UpdateFood(Uuid, Food),
    AddRecipe(Recipe),
    RemoveRecipe(Uuid),
    UpdateRecipe(Uuid, Recipe),
    AddPortion(Portion),
    RemovePortion(Uuid),
    UpdatePortion(Uuid, Portion),
}

#[derive(Debug)]
pub enum AppMessage {
}

pub struct AppModel {
    state: AppState,
    data: AppData,
    header: Controller<HeaderModel>,
    food_page: Controller<FoodPageModel>,
    recipe_page: Controller<RecipePageModel>,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = AppState;
    type Input = AppCommand;
    type Output = AppMessage;

    view! {
        gtk::Window {
            set_title: Some("Chef"),
            set_titlebar: Some(model.header.widget()),
            connect_close_request[sender] => move |_| {
                sender.input(AppCommand::CloseRequest);
                gtk::glib::Propagation::Stop
            },

            #[name(main_stack)]
            gtk::Stack {
                #[watch]
                set_visible_child_name: model.state.page.as_ref(),
                // connect_visible_child_notify[sender] => move |stack| {
                //     if let Some(page_name) = stack.visible_child_name() {
                //         println!("{}", page_name);
                //         match String::from(page_name).as_ref() {
                //             "food_page" => {},
                //             "recipe_page" => {
                //                 sender.input(AppCommand::)
                //             },
                //             _ => {}
                //         }
                //     }
                // },
                add_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    model.food_page.widget(),
                } -> {
                    set_name: "food_page"
                },
                add_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    model.recipe_page.widget(),
                } -> {
                    set_name: "recipe_page",
                },
            }
        }
    }

    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // let state = AppState::default();
        let header: Controller<HeaderModel> = HeaderModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                header::Tab::Food => 
                    AppCommand::SetMode(AppMode::FoodInventory),
                header::Tab::Recipe =>
                    AppCommand::SetMode(AppMode::Recipes)
            });
        let food_page = FoodPageModel::builder()
            .launch(FoodPageState::default())
            .forward(sender.input_sender(), |msg| match msg {
                FoodPageMessage::NoMessage => {
                    AppCommand::NoCommand
                }               
                FoodPageMessage::CommitFood(food) => {
                    AppCommand::AddFood(food)
                }
                FoodPageMessage::CommitFoodRemoval(id) => {
                    AppCommand::RemoveFood(id)
                }
                FoodPageMessage::CommitFoodUpdate(id, food) => {
                    AppCommand::UpdateFood(id, food)
                }
            });
        let recipe_page = RecipePageModel::builder()
            .launch(RecipePageState::default())
            .forward(sender.input_sender(), |msg| match msg {
                RecipePageMessage::NoMessage => {
                    AppCommand::NoCommand
                }               
                RecipePageMessage::CommitRecipe(recipe) => {
                    AppCommand::AddRecipe(recipe)
                }
                RecipePageMessage::CommitRecipeRemoval(id) => {
                    AppCommand::RemoveRecipe(id)
                }
                RecipePageMessage::CommitRecipeUpdate(id, recipe) => {
                    AppCommand::UpdateRecipe(id, recipe)
                }
                RecipePageMessage::CommitPortion(portion) => {
                    AppCommand::AddPortion(portion.inner)
                }
                RecipePageMessage::CommitPortionRemoval(index) => {
                    AppCommand::RemovePortion(index)
                }
                RecipePageMessage::CommitPortionUpdate(index, portion) => {
                    AppCommand::UpdatePortion(index, portion.inner)
                }
            });
        
        let data = AppData::default();        
        sender.input(AppCommand::LoadDatabase);

        let model = AppModel {
            state: init, data,
            header, food_page, recipe_page
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppCommand::CloseRequest => {
                sender.input(AppCommand::PersistDatabase);
                relm4::main_application().quit();
            }
            AppCommand::SetMode(mode) => {
                match mode {
                    AppMode::FoodInventory => {
                        self.state.page = "food_page".to_owned();
                    }
                    AppMode::Recipes => {
                        self.state.page = "recipe_page".to_owned();
                        //reload food ing combobox
                        self.recipe_page.emit(
                            RecipePageCommand::LoadFoodIngredientList(
                                self.data.cuisine.food_list()
                            )
                        )
                    }
                }
            }
            AppCommand::LoadDatabase => {
                self.data = AppData::from_file(self.state.database_path.clone())
                    .unwrap_or_default();

                self.food_page.emit(
                    FoodPageCommand::LoadFoodList(
                        self.data.cuisine.food_list()
                    )
                );
                self.recipe_page.emit(
                    RecipePageCommand::LoadRecipeList(
                        self.data.cuisine.recipe_list()
                    )
                );
                self.recipe_page.emit(
                    RecipePageCommand::LoadFoodPortionList(
                        self.data.cuisine.food_portion_list()
                    )
                );
            }
            AppCommand::PersistDatabase => {
                self.data.to_file(self.state.database_path.clone())
                    .expect("failed saving database");
            }
            AppCommand::AddFood(food) => {
                let id = Uuid::new_v4();
                let food = Food { id, ..food };
                // self.data.foodlist.push(food);
                self.data.cuisine.insert_food(id, food);
            }
            AppCommand::RemoveFood(id) => {
                // dbg!(index);
                // self.data.foodlist.remove(&id);
                self.data.cuisine.remove_food(&id);
            }
            AppCommand::UpdateFood(id, food) => {
                // self.data.foodlist.remove(index);
                self.data.cuisine.insert_food(id, food);
            }
            AppCommand::AddRecipe(recipe) => {
                // self.data.recipelist.push(recipe);
                let id = Uuid::new_v4();
                let recipe = Recipe { id, ..recipe };
                self.data.cuisine.insert_recipe(id, recipe);
            }
            AppCommand::RemoveRecipe(id) => {
                self.data.cuisine.remove_food(&id);
            }
            AppCommand::UpdateRecipe(id, recipe) => {
                self.data.cuisine.insert_recipe(id, recipe);
            }
            AppCommand::AddPortion(portion) => {
                let id = Uuid::new_v4();
                let portion = Portion { id , ..portion };
                self.data.cuisine.insert_portion(id, portion);
            }
            AppCommand::RemovePortion(id) => {
                self.data.cuisine.remove_recipe(&id);
            }
            AppCommand::UpdatePortion(id, portion) => {
                self.data.cuisine.insert_portion(id, portion);
            }
            AppCommand::NoCommand => {}
        }
    }
}
