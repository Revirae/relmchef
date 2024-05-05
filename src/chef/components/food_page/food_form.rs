use relm4::{adw, gtk, prelude::ComponentSender};
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt,
    EditableExt, ListBoxRowExt,
    ActionExt
};
use adw::prelude::{
    PreferencesRowExt,
    EntryRowExt,
    ActionRowExt,
};

use relm4::{Component, ComponentParts, SimpleComponent};
use relm4::{RelmContainerExt, RelmSetChildExt};

use crate::chef::models;


#[derive(Debug)]
pub struct FoodFormModel {
    state: models::Food,
    // buffer: models::Food,
}

#[derive(Default, Debug)]
pub enum FoodFormCommand {
    #[default]
    NoCommand,
    Send,
    ChangeName(String),
    ChangeBrand(String),
    ChangeCost(f64),
    ChangeWeight(f64),
    ChangeVolume(f64),
}

#[derive(Default, Debug)]
pub enum FoodFormMessage {
    #[default]
    NoMessage,
    Changed,
    Submit(models::Food),
}

#[relm4::component(pub)]
impl Component for FoodFormModel {
    type Init = models::Food;
    type Input = FoodFormCommand;
    type Output = FoodFormMessage;
    type CommandOutput = ();
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                #[name(name_entry)]
                adw::EntryRow {
                    set_title: "Nome",
                    set_hexpand: true,
                    connect_changed[sender] => move |entry| {
                        let name = entry.text().to_string();
                        sender.input(
                            FoodFormCommand::ChangeName(name)
                        )
                    }
                },
                #[name(brand_entry)]
                adw::EntryRow {
                    set_title: "Marca",
                    connect_changed[sender] => move |entry| {
                        let brand = entry.text().to_string();
                        sender.input(
                            FoodFormCommand::ChangeBrand(brand)
                        )
                    }
                },
                gtk::Button {
                    connect_clicked[sender] => move |_| {
                        sender.input(FoodFormCommand::Send)
                    }
                },
            },
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                #[name(cost_entry)]
                adw::SpinRow {
                    set_hexpand: true,
                    set_title: "<span size='x-small'>Preço</span>",
                    set_digits: 2,
                    set_adjustment: Some(&gtk::Adjustment::new(
                        0., 0., 9999., 0.05, 0.5, 10.
                    )),
                    connect_changed[sender] => move |entry| {
                        let cost = entry.value();
                        sender.input(
                            FoodFormCommand::ChangeCost(cost)
                        )
                    }
                },
                #[name(weight_entry)]
                adw::SpinRow {
                    set_hexpand: true,
                    set_title: "<span size='x-small'>Peso</span>",
                    set_digits: 2,
                    set_adjustment: Some(&gtk::Adjustment::new(
                        0., 0., 9999., 0.05, 0.5, 10.
                    )),
                    connect_changed[sender] => move |entry| {
                        let weight = entry.value();
                        sender.input(
                            FoodFormCommand::ChangeWeight(weight)
                        )
                    }
                },
                #[name(volume_entry)]
                adw::SpinRow {
                    set_hexpand: true,
                    set_title: "<span size='x-small'>Volume</span>",
                    set_digits: 2,
                    set_activatable: true,
                    set_adjustment: Some(&gtk::Adjustment::new(
                        0., 0., 9999., 0.05, 0.5, 10.
                    )),
                    connect_changed[sender] => move |entry| {
                        let volume = entry.value();
                        sender.input(
                            FoodFormCommand::ChangeVolume(volume)
                        )
                    },
                },
            }
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let state = models::Food::default();
        let buffer = models::Food::default();
        let model = FoodFormModel { state };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            FoodFormCommand::NoCommand => {}
            FoodFormCommand::Send => {
                // todo!("validation");
                sender.output(FoodFormMessage::Submit(
                    self.state.clone()
                ));
            }
            FoodFormCommand::ChangeName(text) => {
                self.state.name = text;
            }
            FoodFormCommand::ChangeBrand(text) => {
                self.state.brand = text;
            }
            FoodFormCommand::ChangeCost(value) => {
                self.state.cost = value;
            }
            FoodFormCommand::ChangeWeight(value) => {
                self.state.weight = value;
            }
            FoodFormCommand::ChangeVolume(value) => {
                self.state.volume = value;
            }
        }
    }
}
