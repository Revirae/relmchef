use relm4::gtk;
use relm4::{Component, ComponentParts, ComponentSender};

use crate::chef::models;


#[derive(Debug)]
pub struct RecipeFormModel {
    state: models::Recipe,
}

#[derive(Default, Debug)]
pub enum RecipeFormMessage {
    #[default]
    NoMessage,
}

#[relm4::component(pub)]
impl Component for RecipeFormModel {
    type Init = models::Recipe;
    type Input = ();
    type Output = RecipeFormMessage;
    type CommandOutput = ();
    view! {
        #[root]
        gtk::Box {}
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = RecipeFormModel { state: init };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
