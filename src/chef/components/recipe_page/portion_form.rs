use relm4::{gtk, prelude::ComponentSender};
use gtk::prelude::{
    ButtonExt,
    WidgetExt, OrientableExt,
};

use relm4::{Component, ComponentParts};

use crate::chef::models;

#[derive(Debug)]
pub struct PortionFormModel {
    state: models::FoodPortion,
    ingredient_list: gtk::StringList,
}

#[derive(Default, Debug)]
pub enum PortionFormMessage {
    #[default]
    NoMessage,
    Submit(models::FoodPortion),
}

#[derive(Default, Debug)]
pub enum PortionFormCommand {
    #[default]
    NoCommand,
    Send,
    Receive(models::FoodPortion),
    ReceiveFoodList(Vec<models::Food>),
    // ChangeName(String),
}

#[derive(Default, Debug)]
pub enum PortionFormAction {
    #[default]
    NoAction,
    Fill,
}

#[relm4::component(pub)]
impl Component for PortionFormModel {
    type Init = models::FoodPortion;
    type Input = PortionFormCommand;
    type Output = PortionFormMessage;
    type CommandOutput = PortionFormAction;
    view! {
        #[root]
        gtk::Box {           
            set_orientation: gtk::Orientation::Horizontal,
            #[name(name_entry)]
            gtk::DropDown {
                #[watch]
                set_model: Some(&model.ingredient_list),
            },
            #[name(send_button)]
            gtk::Button {
                set_icon_name: "document-new",
                set_size_request: (50, 32),
                connect_clicked[sender] => move |_| {
                    sender.input(PortionFormCommand::Send)
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
        let model = PortionFormModel { state: init, ingredient_list };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            PortionFormCommand::NoCommand => {}
            PortionFormCommand::Send => {
                // todo!("validation");
                sender.output(PortionFormMessage::Submit(
                    self.state.clone()
                )).expect("failed to submit form");
            }
            PortionFormCommand::Receive(portion) => {
                // dbg!(portion.clone());
                self.state = portion;
                sender.spawn_command(|sender|
                    sender.emit(PortionFormAction::Fill)
                );
            }
            PortionFormCommand::ReceiveFoodList(food_list) => {
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
