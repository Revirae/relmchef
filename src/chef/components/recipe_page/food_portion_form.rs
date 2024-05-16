use std::collections::HashMap;

use relm4::{gtk, adw, prelude::ComponentSender};
use gtk::prelude::{
    ButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};

use adw::prelude::PreferencesRowExt;

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
    ChangeAmount(f64),
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
            #[name(amount_entry)]
            adw::SpinRow {
                set_hexpand: true,
                set_title: "<span size='x-small'>Qtd.</span>",
                set_digits: 2,
                set_adjustment: Some(&gtk::Adjustment::new(
                    0., 0., 9999., 0.05, 0.5, 10.
                )),
                connect_changed[sender] => move |entry| {
                    let amount = entry.value();
                    sender.input(
                        FoodPortionFormCommand::ChangeAmount(amount)
                    )
                }
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
                let maybe_position = self.food_list.iter().position(|food| {
                    food.id == self.state.inner.ingredient_id
                });
                if let Some(position) = maybe_position {
                    dbg!(position);
                    widgets.name_entry.set_selected(position as u32);
                }
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
                let food = self.food_list.get(index).unwrap();
                self.state.set_ingredient(food);
                dbg!(food.clone());
            }
            FoodPortionFormCommand::ChangeAmount(amount) => {
                self.state.inner.amount_w = amount;
            }
        }
    }
}
