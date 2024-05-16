use relm4::{gtk, prelude::ComponentSender};
use gtk::prelude::{
    ButtonExt,
    WidgetExt, OrientableExt,
};

use relm4::{Component, ComponentParts};

use crate::chef::models;

#[derive(Debug)]
pub struct RecipePortionFormModel {
    state: models::RecipePortion,
    ingredient_list: gtk::StringList,
}

#[derive(Default, Debug)]
pub enum RecipePortionFormMessage {
    #[default]
    NoMessage,
    Submit(models::RecipePortion),
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub enum RecipePortionFormCommand {
    #[default]
    NoCommand,
    Send,
    Receive(models::RecipePortion),
    ReceiveRecipeList(Vec<models::Recipe>),
    // ChangeName(String),
}

#[derive(Default, Debug)]
pub enum RecipePortionFormAction {
    #[default]
    NoAction,
    Fill,
}

#[relm4::component(pub)]
impl Component for RecipePortionFormModel {
    type Init = models::RecipePortion;
    type Input = RecipePortionFormCommand;
    type Output = RecipePortionFormMessage;
    type CommandOutput = RecipePortionFormAction;
    view! {
        #[root]
        gtk::Box {           
            set_orientation: gtk::Orientation::Horizontal,
            #[name(name_entry)]
            gtk::DropDown {
                #[watch]
                set_model: Some(&model.ingredient_list),
                set_hexpand: true,
            },
            #[name(send_button)]
            gtk::Button {
                set_icon_name: "document-new",
                set_size_request: (50, 32),
                connect_clicked[sender] => move |_| {
                    sender.input(RecipePortionFormCommand::Send)
                }
            },
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let ingredient_list = gtk::StringList::default();
        let model = RecipePortionFormModel { state: init, ingredient_list };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            RecipePortionFormCommand::NoCommand => {}
            RecipePortionFormCommand::Send => {
                // todo!("validation");
                sender.output(RecipePortionFormMessage::Submit(
                    self.state.clone()
                )).expect("failed to submit form");
            }
            RecipePortionFormCommand::Receive(portion) => {
                // dbg!(portion.clone());
                self.state = portion;
                sender.spawn_command(|sender|
                    sender.emit(RecipePortionFormAction::Fill)
                );
            }
            RecipePortionFormCommand::ReceiveRecipeList(recipe_list) => {
                dbg!(recipe_list.clone());
                for recipe in recipe_list {
                    self.ingredient_list.append(&recipe.name);
                }
            }
            // PortionFormCommand::ChangeName(text) => {
                // self.state.ingredient.name = text;
            // }
        }
    }
}
