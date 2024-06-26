use relm4::{adw, gtk, prelude::ComponentSender};
use gtk::prelude::{
    ButtonExt,
    WidgetExt, OrientableExt,
    EditableExt, ListBoxRowExt,
};
use adw::prelude::PreferencesRowExt;

use relm4::{Component, ComponentParts};

use crate::chef::models;

  
#[derive(Debug)]
pub struct FoodFormModel {
    state: models::Food,
}

#[derive(Default, Debug)]
pub enum FoodFormCommand {
    #[default]
    NoCommand,
    Send,
    Receive(models::Food),
    ChangeIcon(String),
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
    // Changed,
    Submit(models::Food),
}

#[derive(Default, Debug)]
pub enum FoodFormAction {
    #[default]
    NoAction,
    Fill,
    ChangeButtonIcon(String),
}

#[relm4::component(pub)]
impl Component for FoodFormModel {
    type Init = models::Food;
    type Input = FoodFormCommand;
    type Output = FoodFormMessage;
    type CommandOutput = FoodFormAction;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                #[name(name_entry)]
                adw::EntryRow {
                    set_title: "Nome do alimento",
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
                #[name(send_button)]
                gtk::Button {
                    set_icon_name: "document-new",
                    set_size_request: (50, 32),
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
        let model = FoodFormModel { state: init };
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
            FoodFormAction::Fill => {
                widgets.name_entry.set_text(&self.state.name);
                widgets.brand_entry.set_text(&self.state.brand);
                widgets.cost_entry.set_value(self.state.cost);
                widgets.weight_entry.set_value(self.state.weight);
                widgets.volume_entry.set_value(self.state.volume);
            }
            FoodFormAction::ChangeButtonIcon(icon_code) => {
                widgets.send_button.set_icon_name(&icon_code);
            }
            FoodFormAction::NoAction => {}
        }
    }
    
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            FoodFormCommand::NoCommand => {}
            FoodFormCommand::Send => {
                // todo!("validation");
                sender.output(FoodFormMessage::Submit(
                    self.state.clone()
                )).expect("failed to submit form");
            }
            FoodFormCommand::Receive(food) => {
                dbg!(food.clone());
                self.state = food;
                sender.spawn_command(|sender|
                    sender.emit(FoodFormAction::Fill)
                );
            }
            FoodFormCommand::ChangeIcon(color_code) => {
                sender.spawn_command(|sender| {
                    sender.emit(FoodFormAction::ChangeButtonIcon(color_code))
                });
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
