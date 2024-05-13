// #![allow(unused)]
mod recipe_form;
mod recipe_list;
mod portion_form;
mod portion_list;

use relm4::gtk;
use relm4::prelude::ComponentSender;
use relm4::{
    ComponentParts,
    SimpleComponent,
    Controller,
    ComponentController,
    Component
};
use gtk::prelude::OrientableExt;
use uuid::Uuid;

use crate::chef::models::FoodPortion;
use crate::chef::{components, models};
use components::recipe_page::portion_form::PortionFormMessage;
use components::recipe_page::portion_list::{PortionListMessage, PortionListModel, PortionListState};
use components::recipe_page::recipe_form::RecipeFormMessage;
use components::recipe_page::recipe_list::{RecipeListMessage, RecipeListState};

use models::{Portion, Recipe};

use self::portion_form::{PortionFormCommand, PortionFormModel};
use self::recipe_form::{RecipeFormCommand, RecipeFormModel};

use self::portion_list::PortionListCommand;
use self::recipe_list::{RecipeListCommand, RecipeListModel};


#[derive(Default, Debug)]
pub enum RecipePageMode {
    #[default]
    InsertingRecipe,
    InsertingPortion,
    EditingRecipe(usize),
    EditingPortion(usize),
    // Filtering,
}

#[derive(Default, Debug)]
pub struct RecipePageState {
    mode: RecipePageMode,
    recipelist: Vec<models::Recipe>,
    portionlist: Vec<models::FoodPortion>,
}

#[derive(Debug)]
pub struct RecipePageModel {
    state: RecipePageState,
    recipe_form: Controller<RecipeFormModel>,
    recipe_list: Controller<RecipeListModel>,
    portion_form: Controller<PortionFormModel>,
    portion_list: Controller<PortionListModel>,
}

#[derive(Default, Debug)]
pub enum RecipePageCommand {
    #[default]
    NoCommand,

    LoadRecipeList(Vec<models::Recipe>),
    PutRecipe(models::Recipe),
    RemoveRecipe(usize),
    UpdateRecipe(usize),

    LoadPortionList(Vec<models::FoodPortion>),
    PutPortion(models::FoodPortion),
    RemovePortion(usize),
    UpdatePortion(usize),
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
            model.recipe_form.widget(),           
            model.recipe_list.widget(),
            model.portion_form.widget(),
            model.portion_list.widget(),
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
        let portion_form = PortionFormModel::builder()
            .launch(FoodPortion::default())
            .forward(sender.input_sender(), |msg| match msg {
                PortionFormMessage::NoMessage => {
                    RecipePageCommand::NoCommand
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
            });
        let portion_list = PortionListModel::builder()
            .launch(PortionListState::default())
            .forward(sender.input_sender(), |msg| match msg {
                PortionListMessage::NoMessage => {
                    RecipePageCommand::NoCommand
                }
                PortionListMessage::RequestRemoval(index) => {
                    RecipePageCommand::RemovePortion(index)
                }
                PortionListMessage::RequestUpdate(index) => {
                    RecipePageCommand::UpdatePortion(index)
                }
            });

        
        let model = RecipePageModel  {
            state: init,
            recipe_form,
            portion_form,
            recipe_list,
            portion_list
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            RecipePageCommand::NoCommand => {}
            RecipePageCommand::LoadRecipeList(recipelist) => {
                self.state.recipelist = recipelist.clone();
                for recipe in recipelist {
                    self.recipe_list.emit(
                        RecipeListCommand::AddEntry(recipe)
                    );
                }
            }    
            RecipePageCommand::LoadPortionList(portionlist) => {
                self.state.portionlist = portionlist.clone();
                for portion in portionlist {
                    self.portion_list.emit(
                        PortionListCommand::AddEntry(portion)
                    );
                }
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
                        self.state.mode = RecipePageMode::InsertingRecipe;

                        // self.recipe_form.emit(
                            // RecipeFormCommand::ChangeIcon("document-new".into())
                        // );
                    }
                    RecipePageMode::InsertingRecipe => {
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
            RecipePageCommand::PutPortion(portion) => {
                match self.state.mode {
                    RecipePageMode::EditingPortion(index) => {
                        self.portion_list.emit(
                            PortionListCommand::InsertEntry(index, portion.clone())
                        );
                        sender.output(
                            RecipePageMessage::CommitPortionUpdate(portion.inner.id, portion)
                        ).expect("failed to commit portion update");
                        self.state.mode = RecipePageMode::InsertingRecipe;

                        // self.recipe_form.emit(
                            // PortionFormCommand::ChangeIcon("document-new".into())
                        // );
                    }
                    RecipePageMode::InsertingPortion => {
                        self.state.portionlist.push(portion.clone());
                        self.portion_list.emit(
                            PortionListCommand::AddEntry(portion.clone())
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
                let portion= self.state.portionlist.get(index).unwrap();
                self.portion_form.emit(
                    PortionFormCommand::Receive(portion.clone().inner)
                );
                self.state.mode = RecipePageMode::EditingRecipe(index);
                // self.recipe_form.emit(
                    // PortionFormCommand::ChangeIcon("document-save".into())
                // );
            }
        }
    }
}
