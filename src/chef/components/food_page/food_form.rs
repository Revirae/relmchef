use std::cell::{Cell, RefCell};
use std::rc::Rc;

use relm4::gtk::glib::value::ToValue;
use relm4::{adw, gtk, prelude::ComponentSender};
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};
use adw::prelude::PreferencesRowExt;

use relm4::{Component, ComponentParts, SimpleComponent};
use relm4::{RelmContainerExt, RelmSetChildExt};

use crate::chef::models;


#[derive(Debug)]
pub struct FoodFormModel {
    state: models::Food,
}

#[derive(Default, Debug)]
pub enum FoodFormCommand {
    #[default]
    NoCommand,
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
impl SimpleComponent for FoodFormModel {
    type Init = models::Food;
    type Input = FoodFormCommand;
    type Output = FoodFormMessage;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            #[name(name_entry)]
            adw::EntryRow {
                #[watch]
                set_text: model.state.name.as_ref(),
                set_title: "Nome",
            },
            #[name(brand_entry)]
            adw::EntryRow {
                #[watch]
                set_text: model.state.brand.as_ref(),
                set_title: "Marca",
            },
            #[name(cost_entry)]
            adw::SpinRow {
                set_title: "Custo",
                #[watch]
                set_value: model.state.cost,
                set_digits: 2,
                set_adjustment: Some(&gtk::Adjustment::new(
                    0., 0., 9999., 0.05, 0.5, 10.
                )),
            },
            #[name(weight_entry)]
            adw::SpinRow {
                set_title: "Peso",
                #[watch]
                set_value: model.state.weight,
                set_digits: 2,
                set_adjustment: Some(&gtk::Adjustment::new(
                    0., 0., 9999., 0.05, 0.5, 10.
                )),
            },
            #[name(volume_entry)]
            adw::SpinRow {
                set_title: "Volume",
                #[watch]
                set_value: model.state.volume,
                set_digits: 2,
                set_adjustment: Some(&gtk::Adjustment::new(
                    0., 0., 9999., 0.05, 0.5, 10.
                )),
            },
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let state = models::Food::default();
        let model = FoodFormModel { state };
        let widgets = view_output!();

        let _sender = sender.clone();
        widgets.name_entry.connect_changed(move |entry| {
            _sender.input(FoodFormCommand::ChangeName(
                entry.text().to_string()
            ));
        });       

        let _sender = sender.clone();
        widgets.brand_entry.connect_changed(move |entry| {
            _sender.input(FoodFormCommand::ChangeBrand(
                entry.text().to_string()
            ))
        });       

        let _sender = sender.clone();
        widgets.cost_entry.connect_changed(move |entry| {
            _sender.input(FoodFormCommand::ChangeCost(
                entry.value()
            ))
        });

        let _sender = sender.clone();
        widgets.weight_entry.connect_changed(move |entry| {
            _sender.input(FoodFormCommand::ChangeWeight(
                entry.value()
            ))
        });

        let _sender = sender.clone();
        widgets.volume_entry.connect_changed(move |entry| {
            _sender.input(FoodFormCommand::ChangeVolume(
                entry.value()
            ))
        });

        ComponentParts { model, widgets }
    }
    fn update_cmd(&mut self, input: &relm4::Sender<Self::Input>, output: relm4::Sender<Self::Output>) {
        // match output {
        //     FoodFormMessage::ChangedName(name) => {
        //     }
        // }
    }
    // fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {
    // }
    
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            FoodFormCommand::NoCommand => {}
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
