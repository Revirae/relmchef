use relm4::{gtk, prelude::ComponentSender};
use gtk::prelude::{
    ButtonExt,
    WidgetExt, OrientableExt,
};

use relm4::{Component, ComponentParts};

use crate::chef::models;

#[derive(Debug)]
pub struct FoodPortionFormModel {
    state: models::FoodPortion,
    ingredient_list: gtk::StringList,
}

#[derive(Default, Debug)]
pub enum FoodPortionFormMessage {
    #[default]
    NoMessage,
    Submit(models::FoodPortion),
}

#[derive(Default, Debug)]
pub enum FoodPortionFormCommand {
    #[default]
    NoCommand,
    Send,
    Receive(models::FoodPortion),
    ReceiveFoodList(Vec<models::Food>),
    // ChangeName(String),
}

#[derive(Default, Debug)]
pub enum FoodPortionFormAction {
    #[default]
    NoAction,
    Fill,
}

#[relm4::component(pub)]
impl Component for FoodPortionFormModel {
    type Init = models::FoodPortion;
    type Input = FoodPortionFormCommand;
    type Output = FoodPortionFormMessage;
    type CommandOutput = FoodPortionFormAction;
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
                    sender.input(FoodPortionFormCommand::Send)
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
        let model = FoodPortionFormModel { state: init, ingredient_list };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            FoodPortionFormCommand::NoCommand => {}
            FoodPortionFormCommand::Send => {
                // todo!("validation");
                sender.output(FoodPortionFormMessage::Submit(
                    self.state.clone()
                )).expect("failed to submit form");
            }
            FoodPortionFormCommand::Receive(portion) => {
                // dbg!(portion.clone());
                self.state = portion;
                sender.spawn_command(|sender|
                    sender.emit(FoodPortionFormAction::Fill)
                );
            }
            FoodPortionFormCommand::ReceiveFoodList(food_list) => {
                dbg!(food_list.clone());
                for food in food_list {
                    self.ingredient_list.append(&food.name);
                }
            }
            // PortionFormCommand::ChangeName(text) => {
                // self.state.ingredient.name = text;
            // }
        }
    }
}
