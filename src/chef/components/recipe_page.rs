// #![allow(unused)]
mod recipe_form;
mod recipe_list;
mod food_portion_form;
mod food_portion_list;

use relm4::gtk;
use relm4::prelude::ComponentSender;
use relm4::{
    ComponentParts,
    SimpleComponent,
    Controller,
    ComponentController,
    Component
};
use gtk::prelude::{WidgetExt, OrientableExt};
use uuid::Uuid;

use crate::chef::models::FoodPortion;
use crate::chef::{components, models};

use components::recipe_page::food_portion_form::FoodPortionFormMessage;
use components::recipe_page::food_portion_list::{
    FoodPortionListMessage,
    FoodPortionListModel,
    FoodPortionListState
};
use components::recipe_page::recipe_form::RecipeFormMessage;
use components::recipe_page::recipe_list::{RecipeListMessage, RecipeListState};

use models::Recipe;

use self::food_portion_form::{FoodPortionFormCommand, FoodPortionFormModel};
use self::recipe_form::{RecipeFormCommand, RecipeFormModel};

use self::food_portion_list::FoodPortionListCommand;
use self::recipe_list::{RecipeListCommand, RecipeListModel};


#[derive(Default, Debug)]
pub enum RecipePageMode {
    #[default]
    Inserting,
    EditingRecipe(usize),
    #[allow(dead_code)]
    EditingPortion(usize),
}

#[derive(Default, Debug)]
pub struct RecipePageState {
    mode: RecipePageMode,
    recipelist: Vec<models::Recipe>,
    foodportionlist: Vec<models::FoodPortion>,
}

#[derive(Debug)]
pub struct RecipePageModel {
    state: RecipePageState,
    recipe_form: Controller<RecipeFormModel>,
    recipe_list: Controller<RecipeListModel>,
    food_portion_form: Controller<FoodPortionFormModel>,
    food_portion_list: Controller<FoodPortionListModel>,
}

#[derive(Default, Debug)]
pub enum RecipePageCommand {
    #[default]
    NoCommand,

    LoadRecipeList(Vec<models::Recipe>),
    PutRecipe(models::Recipe),
    RemoveRecipe(usize),
    UpdateRecipe(usize),
    BuildRecipe(usize),

    ReceiveFoodPortionList(Vec<models::FoodPortion>),
    LoadFoodPortionList(Uuid),
    PutPortion(models::FoodPortion),
    RemovePortion(usize),
    UpdatePortion(usize),

    LoadFoodIngredientList(Vec<models::Food>),
}

#[derive(Default, Debug)]
pub enum RecipePageMessage {
    #[default]
    NoMessage,

    CommitRecipe(Recipe),
    CommitRecipeRemoval(Uuid),
    CommitRecipeUpdate(Uuid, Recipe),

    CommitPortion(models::FoodPortion),
    CommitPortionRemoval(Uuid),
    CommitPortionUpdate(Uuid, models::FoodPortion),
}

#[relm4::component(pub)]
impl SimpleComponent for RecipePageModel  {
    type Init = RecipePageState;
    type Input = RecipePageCommand;
    type Output = RecipePageMessage;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    model.recipe_form.widget(),           
                    model.recipe_list.widget(),
                },
                gtk::Box {
                    set_hexpand: true,
                    set_orientation: gtk::Orientation::Vertical,
                    model.food_portion_form.widget(),
                    model.food_portion_list.widget(),
                }
            }
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let recipe_form = RecipeFormModel::builder()
            .launch(Recipe::default())
            .forward(sender.input_sender(), |msg| match msg {
                RecipeFormMessage::NoMessage => {
                    RecipePageCommand::NoCommand
                }
                RecipeFormMessage::Submit(recipe) => {
                    RecipePageCommand::PutRecipe(recipe)    
                }
            });
        let food_portion_form = FoodPortionFormModel::builder()
            .launch(FoodPortion::default())
            .forward(sender.input_sender(), |msg| match msg {
                FoodPortionFormMessage::NoMessage => {
                    RecipePageCommand::NoCommand
                }
                FoodPortionFormMessage::Submit(portion) => {
                    RecipePageCommand::PutPortion(portion)
                }
            });
        
        let recipe_list = RecipeListModel::builder()
            .launch(RecipeListState::default())
            .forward(sender.input_sender(), |msg| match msg {
                RecipeListMessage::NoMessage => {
                    RecipePageCommand::NoCommand
                }
                RecipeListMessage::RequestRemoval(index) => {
                    RecipePageCommand::RemoveRecipe(index)
                }
                RecipeListMessage::RequestUpdate(index) => {
                    RecipePageCommand::UpdateRecipe(index)
                }
                RecipeListMessage::RequestBuilding(index) => {
                    RecipePageCommand::BuildRecipe(index)
                }
            });
        let food_portion_list = FoodPortionListModel::builder()
            .launch(FoodPortionListState::default())
            .forward(sender.input_sender(), |msg| match msg {
                FoodPortionListMessage::NoMessage => {
                    RecipePageCommand::NoCommand
                }
                FoodPortionListMessage::RequestRemoval(index) => {
                    RecipePageCommand::RemovePortion(index)
                }
                FoodPortionListMessage::RequestUpdate(index) => {
                    RecipePageCommand::UpdatePortion(index)
                }
            });
        
        let model = RecipePageModel  {
            state: init,
            recipe_form,
            food_portion_form,
            recipe_list,
            food_portion_list
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            RecipePageCommand::NoCommand => {}
            RecipePageCommand::ReceiveFoodPortionList(portionlist) => {
                self.state.foodportionlist = portionlist.clone();
            }
            RecipePageCommand::LoadRecipeList(recipelist) => {
                dbg!(recipelist.clone());
                self.state.recipelist = recipelist.clone();
                for recipe in recipelist {
                    self.recipe_list.emit(
                        RecipeListCommand::AddEntry(recipe)
                    );
                }
            }    
            RecipePageCommand::LoadFoodPortionList(recipe_id) => {
                let foodportionlist: Vec<FoodPortion> =
                    self.state.foodportionlist
                        .clone()
                        .into_iter()
                        .filter(|portion| portion.inner.recipe_id == recipe_id)
                        .collect();
                self.food_portion_list.emit(FoodPortionListCommand::Clear);
                for portion in foodportionlist {
                    self.food_portion_list.emit(
                        FoodPortionListCommand::AddEntry(portion)
                    );
                }
            }    
            RecipePageCommand::LoadFoodIngredientList(foodlist) => {
                self.food_portion_form.emit(
                    FoodPortionFormCommand::ReceiveFoodList(foodlist)
                );
            }
            RecipePageCommand::PutRecipe(recipe) => {
                match self.state.mode {
                    RecipePageMode::EditingRecipe(index) => {
                        self.recipe_list.emit(
                            RecipeListCommand::InsertEntry(index, recipe.clone())
                        );
                        sender.output(
                            RecipePageMessage::CommitRecipeUpdate(recipe.id, recipe)
                        ).expect("failed to commit recipe update");
                        self.state.mode = RecipePageMode::Inserting;

                        // self.recipe_form.emit(
                            // RecipeFormCommand::ChangeIcon("document-new".into())
                        // );
                    }
                    RecipePageMode::Inserting => {
                        self.state.recipelist.push(recipe.clone());
                        self.recipe_list.emit(
                            RecipeListCommand::AddEntry(recipe.clone())
                        );
                        sender.output(
                            RecipePageMessage::CommitRecipe(recipe)
                        ).expect("failed to commit recipe insertion");
                    }
                    _ => {}
                }
            }
            RecipePageCommand::RemoveRecipe(index) => {
                let id = self.state.recipelist.get(index).unwrap().id;
                self.state.recipelist.remove(index);
                sender.output(
                    RecipePageMessage::CommitRecipeRemoval(id)
                ).expect("failed to commit recipe removal");
            }
            RecipePageCommand::UpdateRecipe(index) => {
                let recipe = self.state.recipelist.get(index).unwrap();
                self.recipe_form.emit(
                    RecipeFormCommand::Receive(recipe.clone())
                );
                self.state.mode = RecipePageMode::EditingRecipe(index);
                // self.recipe_form.emit(
                    // RecipeFormCommand::ChangeIcon("document-save".into())
                // );
            }
            RecipePageCommand::BuildRecipe(index) => {
                let recipe = self.state.recipelist.get(index).unwrap();
                self.food_portion_form.emit(FoodPortionFormCommand::Enable(recipe.id));
                sender.input(RecipePageCommand::LoadFoodPortionList(recipe.id));
            }
            RecipePageCommand::PutPortion(portion) => {
                match self.state.mode {
                    RecipePageMode::EditingPortion(index) => {
                        self.food_portion_list.emit(
                            FoodPortionListCommand::InsertEntry(index, portion.clone())
                        );
                        sender.output(
                            RecipePageMessage::CommitPortionUpdate(portion.inner.id, portion)
                        ).expect("failed to commit portion update");
                        self.state.mode = RecipePageMode::Inserting;

                        // self.recipe_form.emit(
                            // PortionFormCommand::ChangeIcon("document-new".into())
                        // );
                    }
                    RecipePageMode::Inserting => {
                        self.state.foodportionlist.push(portion.clone());
                        self.food_portion_list.emit(
                            FoodPortionListCommand::AddEntry(portion.clone())
                        );
                        sender.output(
                            RecipePageMessage::CommitPortion(portion)
                        ).expect("failed to commit portion insertion");
                    }
                    _ => {}
                }
            }
            RecipePageCommand::RemovePortion(index) => {
                let id = self.state.recipelist.get(index).unwrap().id;
                self.state.recipelist.remove(index);
                sender.output(
                    RecipePageMessage::CommitPortionRemoval(id)
                ).expect("failed to commit portion removal");
            }
            RecipePageCommand::UpdatePortion(index) => {
                let portion= self.state.foodportionlist.get(index).unwrap();
                self.food_portion_form.emit(
                    FoodPortionFormCommand::Receive(portion.clone())
                );
                self.state.mode = RecipePageMode::EditingRecipe(index);
                // self.recipe_form.emit(
                    // PortionFormCommand::ChangeIcon("document-save".into())
                // );
            }
        }
    }
}
