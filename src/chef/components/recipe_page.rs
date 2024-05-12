#![allow(unused)]
mod recipe_form;

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
use crate::chef::models;

use self::recipe_form::RecipeFormModel;


#[derive(Default, Debug)]
pub struct RecipePageState {
    recipelist: Vec<models::Recipe>,
}

#[derive(Debug)]
pub struct RecipePageModel {
    state: RecipePageState,
    recipe_form: Controller<RecipeFormModel>,
}

#[derive(Default, Debug)]
pub enum RecipePageCommand {
    #[default]
    NoCommand,
}

#[relm4::component(pub)]
impl SimpleComponent for RecipePageModel  {
    type Init = models::Recipe;
    type Input = RecipePageCommand;
    type Output = ();
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            model.recipe_form.widget(),           
            // model.food_list.widget(),
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: relm4::prelude::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let recipe_form = RecipeFormModel::builder()
            .launch(init)
            .forward(sender.input_sender(), |msg| match msg {
                RecipeFormMessage::NoMessage => {
                    RecipePageCommand::NoCommand
                }
            });
        
        let state = RecipePageState::default();
        let model = RecipePageModel  {
            state,
            recipe_form,
            // food_list
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
