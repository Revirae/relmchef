#![allow(unused)]
use relm4::gtk;
use relm4::prelude::ComponentSender;
use relm4::prelude::RelmWidgetExt;
use relm4::{
    ComponentParts,
    SimpleComponent,
    Controller,
    ComponentController,
    Component
};
use gtk::prelude::{ WidgetExt, OrientableExt };

use crate::chef::models::{ Recipe, FoodPortion };


#[derive(Debug, Default)]
pub struct RecipeInfoState {
    recipe: Recipe,
    foodportions: Vec<FoodPortion>,
    total_cost: f64,
}

#[derive(Debug, Default)]
pub struct RecipeInfoModel {
    state: RecipeInfoState,
    name_info: String,
    cost_info: String,
}

#[derive(Debug)]
pub enum RecipeInfoCommand {
    Receive(Recipe, Vec<FoodPortion>),
    Make,
}

#[derive(Debug)]
pub enum RecipeInfoMessage {
    
}

#[relm4::component(pub)]
impl SimpleComponent for RecipeInfoModel  {
    type Init = ();
    type Input = RecipeInfoCommand;
    type Output = RecipeInfoMessage;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            gtk::Label {
                set_halign: gtk::Align::Center,
                set_text: "Detalhes",
            },
            gtk::Frame {
                set_height_request: 200,
                gtk::Box {
                    set_margin_all: 25,
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        set_halign: gtk::Align::Start,
                        #[watch]
                        set_text: &model.name_info,
                    },
                    gtk::Label {
                        set_halign: gtk::Align::Start,
                        #[watch]
                        set_text: &model.cost_info,
                    }
                }
            },
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = RecipeInfoModel::default();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            RecipeInfoCommand::Receive(recipe, food_portions) => {
                self.state.recipe = recipe;
                self.state.foodportions = food_portions;
                sender.input(RecipeInfoCommand::Make);
            }
            RecipeInfoCommand::Make => {
                self.name_info = self.state.recipe.name.clone();

                let mut total = 0.0;
                for foodportion in self.state.foodportions.iter() {
                    total += foodportion.inner.amount_w;
                }
                self.cost_info = format!("Total: R$ {total}")
            }
        }
    }
}
