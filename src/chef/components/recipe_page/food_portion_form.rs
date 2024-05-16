use std::collections::HashMap;

use relm4::{gtk, prelude::ComponentSender};
use gtk::prelude::{
    ButtonExt,
    WidgetExt, OrientableExt,
};

use relm4::{Component, ComponentParts};
use uuid::Uuid;

use crate::chef::models;


#[derive(Debug)]
pub struct FoodPortionFormModel {
    state: models::FoodPortion,
    food_list: Vec<models::Food>,
    food_map: HashMap<Uuid, models::Food>,
    food_name_list: gtk::StringList,
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
    Enable(Uuid),
    Send,
    Receive(models::FoodPortion),
    ReceiveFoodList(Vec<models::Food>),
    ChangeSelected(usize),
}

#[derive(Default, Debug)]
pub enum FoodPortionFormAction {
    #[default]
    NoAction,
    Fill,
    EditableEntry(bool),
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
                set_model: Some(&model.food_name_list),
                set_hexpand: true,
                set_sensitive: false,
                connect_selected_notify[sender] => move |dd| {
                    let index = dd.selected() as usize;
                    sender.input(
                        FoodPortionFormCommand::ChangeSelected(index)
                    );
                },
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
        let food_list = Vec::<models::Food>::new();
        let food_map = HashMap::<Uuid, models::Food>::new();
        let food_name_list = gtk::StringList::default();
        let model = FoodPortionFormModel {
            state: init, food_name_list, food_list, food_map
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    fn update_cmd_with_view(
            &mut self,
            widgets: &mut Self::Widgets,
            message: Self::CommandOutput,
            _sender: ComponentSender<Self>,
            _root: &Self::Root,
        ) {
        match message {
            FoodPortionFormAction::Fill => {
                // widgets.name_entry self.state.ingredient.name;
            }
            FoodPortionFormAction::EditableEntry(is_editable) => {
                dbg!(is_editable);
                widgets.name_entry.set_sensitive(is_editable);
            }
            _ => {}
        }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            FoodPortionFormCommand::NoCommand => {}
            FoodPortionFormCommand::Enable(recipe_id) => {
                dbg!(recipe_id);
                self.state.inner.recipe_id = recipe_id;
                sender.spawn_command(|sender| {
                    sender.emit(FoodPortionFormAction::EditableEntry(true))
                });
            }
            FoodPortionFormCommand::Send => {
                // todo!("validation");
                sender.output(FoodPortionFormMessage::Submit(
                    self.state.clone()
                )).expect("failed to submit form");
            }
            FoodPortionFormCommand::Receive(portion) => {
                self.state = portion;
                sender.spawn_command(|sender|
                    sender.emit(FoodPortionFormAction::Fill)
                );
            }
            FoodPortionFormCommand::ReceiveFoodList(food_list) => {
                for food in food_list.iter() {
                    self.food_name_list.append(&food.name);
                self.food_map = food_list
                    .iter()
                    .map(|food| (food.id, food.clone()))
                    .collect();
                }
                self.food_list = food_list;
            }
            FoodPortionFormCommand::ChangeSelected(index) => {
                // dbg!(index);
                // dbg!(self.food_list.clone());
                let food = self.food_list.get(index).unwrap();
                self.state.set_ingredient(food);
                dbg!(food.clone());
            }
        }
    }
}
