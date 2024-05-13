#![allow(unused)]
mod recipe_form;
mod recipe_list;

use relm4::gtk;
use relm4::{
    ComponentParts,
    SimpleComponent,
    Controller,
    ComponentController,
    Component
};
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt
};

use crate::chef::components::recipe_page::recipe_form::RecipeFormMessage;
use crate::chef::components::recipe_page::recipe_list::{RecipeListMessage, RecipeListState};
use crate::chef::models::{self, Recipe};

use self::recipe_form::RecipeFormModel;
use self::recipe_list::RecipeListModel;


#[derive(Default, Debug)]
pub struct RecipePageState {
    recipelist: Vec<models::Recipe>,
}

#[derive(Debug)]
pub struct RecipePageModel {
    state: RecipePageState,
    recipe_form: Controller<RecipeFormModel>,
    recipe_list: Controller<RecipeListModel>,
}

#[derive(Default, Debug)]
pub enum RecipePageCommand {
    #[default]
    NoCommand,
    LoadFoodlist(Vec<models::Recipe>),
    Put(models::Recipe),
    Remove(usize),
    Update(usize),
}

#[derive(Default, Debug)]
pub enum RecipePageMessage {
    #[default]
    NoMessage,
    CommitRecipe(Recipe),
    CommitRecipeRemoval(usize),
    CommitRecipeUpdate(usize, Recipe),
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
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: relm4::prelude::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let recipe_form = RecipeFormModel::builder()
            .launch(Recipe::default())
            .forward(sender.input_sender(), |msg| match msg {
                RecipeFormMessage::NoMessage => {
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
                    RecipePageCommand::Remove(index)
                }
                RecipeListMessage::RequestUpdate(index) => {
                    RecipePageCommand::Update(index)
                }
            });
        let state = RecipePageState::default();
        let model = RecipePageModel  {
            state,
            recipe_form,
            recipe_list
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
